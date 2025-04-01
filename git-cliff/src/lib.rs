//! A highly customizable changelog generator ⛰️
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/img/git-cliff.png",
	html_favicon_url = "https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/favicon/favicon.ico"
)]

/// Command-line argument parser.
pub mod args;

/// Custom logger implementation.
pub mod logger;

#[macro_use]
extern crate log;

use args::{
	BumpOption,
	Opt,
	Sort,
	Strip,
};
use clap::ValueEnum;
use git_cliff_core::changelog::Changelog;
use git_cliff_core::commit::{
	Commit,
	Range,
};
use git_cliff_core::config::{
	CommitParser,
	Config,
};
use git_cliff_core::embed::{
	BuiltinConfig,
	EmbeddedConfig,
};
use git_cliff_core::error::{
	Error,
	Result,
};
use git_cliff_core::release::Release;
use git_cliff_core::repo::{
	Repository,
	SubmoduleRange,
};
use git_cliff_core::{
	DEFAULT_CONFIG,
	IGNORE_FILE,
};
use glob::Pattern;
use std::env;
use std::fs::{
	self,
	File,
};
use std::io;
use std::path::{
	Path,
	PathBuf,
};
use std::time::{
	SystemTime,
	UNIX_EPOCH,
};

/// Checks for a new version on crates.io
#[cfg(feature = "update-informer")]
fn check_new_version() {
	use update_informer::Check;
	let pkg_name = env!("CARGO_PKG_NAME");
	let pkg_version = env!("CARGO_PKG_VERSION");
	let informer = update_informer::new(
		update_informer::registry::Crates,
		pkg_name,
		pkg_version,
	);
	if let Some(new_version) = informer.check_version().ok().flatten() {
		if new_version.semver().pre.is_empty() {
			log::info!(
				"A new version of {pkg_name} is available: v{pkg_version} -> \
				 {new_version}",
			);
		}
	}
}

/// Produces a commit range on the format `BASE..HEAD`, derived from the
/// command line arguments and repository tags.
///
/// If no commit range could be determined, `None` is returned.
fn determine_commit_range(
	args: &Opt,
	config: &Config,
	repository: &Repository,
) -> Result<Option<String>> {
	let tags = repository.tags(
		&config.git.tag_pattern,
		args.topo_order,
		args.use_branch_tags,
	)?;

	let mut commit_range = args.range.clone();
	if args.unreleased {
		if let Some(last_tag) = tags.last().map(|(k, _)| k) {
			commit_range = Some(format!("{last_tag}..HEAD"));
		}
	} else if args.latest || args.current {
		if tags.len() < 2 {
			let commits = repository.commits(
				None,
				None,
				None,
				config.git.topo_order_commits,
			)?;
			if let (Some(tag1), Some(tag2)) = (
				commits.last().map(|c| c.id().to_string()),
				tags.get_index(0).map(|(k, _)| k),
			) {
				if tags.len() == 1 {
					commit_range = Some(tag2.to_owned());
				} else {
					commit_range = Some(format!("{tag1}..{tag2}"));
				}
			}
		} else {
			let mut tag_index = tags.len() - 2;
			if args.current {
				if let Some(current_tag_index) =
					repository.current_tag().as_ref().and_then(|tag| {
						tags.iter()
							.enumerate()
							.find(|(_, (_, v))| v.name == tag.name)
							.map(|(i, _)| i)
					}) {
					match current_tag_index.checked_sub(1) {
						Some(i) => tag_index = i,
						None => {
							return Err(Error::ChangelogError(String::from(
								"No suitable tags found. Maybe run with \
								 '--topo-order'?",
							)));
						}
					}
				} else {
					return Err(Error::ChangelogError(String::from(
						"No tag exists for the current commit",
					)));
				}
			}
			if let (Some(tag1), Some(tag2)) = (
				tags.get_index(tag_index).map(|(k, _)| k),
				tags.get_index(tag_index + 1).map(|(k, _)| k),
			) {
				commit_range = Some(format!("{tag1}..{tag2}"));
			}
		}
	}

	Ok(commit_range)
}

/// Process submodules and add commits to release.
fn process_submodules(
	repository: &'static Repository,
	release: &mut Release,
	topo_order_commits: bool,
) -> Result<()> {
	// Retrieve first and last commit of a release to create a commit range.
	let first_commit = release
		.previous
		.as_ref()
		.and_then(|previous_release| previous_release.commit_id.clone())
		.and_then(|commit_id| repository.find_commit(&commit_id));
	let last_commit = release
		.commit_id
		.clone()
		.and_then(|commit_id| repository.find_commit(&commit_id));

	trace!("Processing submodule commits in {first_commit:?}..{last_commit:?}");

	// Query repository for submodule changes. For each submodule a
	// SubmoduleRange is created, describing the range of commits in the context
	// of that submodule.
	if let Some(last_commit) = last_commit {
		let submodule_ranges =
			repository.submodules_range(first_commit, last_commit)?;
		let submodule_commits =
			submodule_ranges.iter().filter_map(|submodule_range| {
				// For each submodule, the commit range is exploded into a list of
				// commits.
				let SubmoduleRange {
					repository: sub_repo,
					range: range_str,
				} = submodule_range;
				let commits = sub_repo
					.commits(Some(range_str), None, None, topo_order_commits)
					.ok()
					.map(|commits| commits.iter().map(Commit::from).collect());

				let submodule_path = sub_repo.path().to_string_lossy().into_owned();
				Some(submodule_path).zip(commits)
			});
		// Insert submodule commits into map.
		for (submodule_path, commits) in submodule_commits {
			release.submodule_commits.insert(submodule_path, commits);
		}
	}
	Ok(())
}

/// Processes the tags and commits for creating release entries for the
/// changelog.
///
/// This function uses the configuration and arguments to process the given
/// repository individually.
fn process_repository<'a>(
	repository: &'static Repository,
	config: &mut Config,
	args: &Opt,
) -> Result<Vec<Release<'a>>> {
	let mut tags = repository.tags(
		&config.git.tag_pattern,
		args.topo_order,
		args.use_branch_tags,
	)?;
	let skip_regex = config.git.skip_tags.as_ref();
	let ignore_regex = config.git.ignore_tags.as_ref();
	let count_tags = config.git.count_tags.as_ref();
	let recurse_submodules = config.git.recurse_submodules.unwrap_or(false);
	tags.retain(|_, tag| {
		let name = &tag.name;

		// Keep skip tags to drop commits in the later stage.
		let skip = skip_regex.is_some_and(|r| r.is_match(name));
		if skip {
			return true;
		}

		let count = count_tags.is_none_or(|r| {
			let count_tag = r.is_match(name);
			if count_tag {
				trace!("Counting release: {}", name);
			}
			count_tag
		});

		let ignore = ignore_regex.is_some_and(|r| {
			if r.as_str().trim().is_empty() {
				return false;
			}

			let ignore_tag = r.is_match(name);
			if ignore_tag {
				trace!("Ignoring release: {}", name);
			}
			ignore_tag
		});

		count && !ignore
	});

	if !config.remote.is_any_set() {
		match repository.upstream_remote() {
			Ok(remote) => {
				if !config.remote.github.is_set() {
					debug!("No GitHub remote is set, using remote: {}", remote);
					config.remote.github.owner = remote.owner;
					config.remote.github.repo = remote.repo;
					config.remote.github.is_custom = remote.is_custom;
				} else if !config.remote.gitlab.is_set() {
					debug!("No GitLab remote is set, using remote: {}", remote);
					config.remote.gitlab.owner = remote.owner;
					config.remote.gitlab.repo = remote.repo;
					config.remote.gitlab.is_custom = remote.is_custom;
				} else if !config.remote.gitea.is_set() {
					debug!("No Gitea remote is set, using remote: {}", remote);
					config.remote.gitea.owner = remote.owner;
					config.remote.gitea.repo = remote.repo;
					config.remote.gitea.is_custom = remote.is_custom;
				} else if !config.remote.bitbucket.is_set() {
					debug!("No Bitbucket remote is set, using remote: {}", remote);
					config.remote.bitbucket.owner = remote.owner;
					config.remote.bitbucket.repo = remote.repo;
					config.remote.bitbucket.is_custom = remote.is_custom;
				}
			}
			Err(e) => {
				debug!("Failed to get remote from repository: {:?}", e);
			}
		}
	}
	if args.use_native_tls {
		config.remote.enable_native_tls();
	}

	// Print debug information about configuration and arguments.
	log::trace!("Arguments: {:#?}", args);
	log::trace!("Config: {:#?}", config);

	// Parse commits.
	let commit_range = determine_commit_range(args, config, repository)?;

	// Include only the current directory if not running from the root repository
	let mut include_path = args.include_path.clone();
	if let Some(mut path_diff) =
		pathdiff::diff_paths(env::current_dir()?, repository.root_path()?)
	{
		if args.workdir.is_none() &&
			include_path.is_none() &&
			path_diff != Path::new("")
		{
			info!(
				"Including changes from the current directory: {:?}",
				path_diff.display()
			);
			path_diff.extend(["**", "*"]);
			include_path =
				Some(vec![Pattern::new(path_diff.to_string_lossy().as_ref())?]);
		}
	}

	let mut commits = repository.commits(
		commit_range.as_deref(),
		include_path,
		args.exclude_path.clone(),
		config.git.topo_order_commits,
	)?;
	if let Some(commit_limit_value) = config.git.limit_commits {
		commits.truncate(commit_limit_value);
	}

	// Update tags.
	let mut releases = vec![Release::default()];
	let mut tag_timestamp = None;
	if let Some(ref tag) = args.tag {
		if let Some(commit_id) = commits.first().map(|c| c.id().to_string()) {
			match tags.get(&commit_id) {
				Some(tag) => {
					warn!("There is already a tag ({}) for {}", tag.name, commit_id);
					tag_timestamp = Some(commits[0].time().seconds());
				}
				None => {
					tags.insert(commit_id, repository.resolve_tag(tag));
				}
			}
		} else {
			releases[0].version = Some(tag.to_string());
			releases[0].timestamp = SystemTime::now()
				.duration_since(UNIX_EPOCH)?
				.as_secs()
				.try_into()?;
		}
	}

	// Process releases.
	let mut previous_release = Release::default();
	let mut first_processed_tag = None;
	let repository_path = repository.root_path()?.to_string_lossy().into_owned();
	for git_commit in commits.iter().rev() {
		let release = releases.last_mut().unwrap();
		let commit = Commit::from(git_commit);
		let commit_id = commit.id.to_string();
		release.commits.push(commit);
		release.repository = Some(repository_path.clone());
		release.commit_id = Some(commit_id);
		if let Some(tag) = tags.get(release.commit_id.as_ref().unwrap()) {
			release.version = Some(tag.name.to_string());
			release.message.clone_from(&tag.message);
			release.timestamp = if args.tag.as_deref() == Some(tag.name.as_str()) {
				match tag_timestamp {
					Some(timestamp) => timestamp,
					None => SystemTime::now()
						.duration_since(UNIX_EPOCH)?
						.as_secs()
						.try_into()?,
				}
			} else {
				git_commit.time().seconds()
			};
			if first_processed_tag.is_none() {
				first_processed_tag = Some(tag);
			}
			previous_release.previous = None;
			release.previous = Some(Box::new(previous_release));
			previous_release = release.clone();
			releases.push(Release::default());
		}
	}

	debug_assert!(!releases.is_empty());

	if releases.len() > 1 {
		previous_release.previous = None;
		releases.last_mut().unwrap().previous = Some(Box::new(previous_release));
	}

	if args.sort == Sort::Newest {
		for release in &mut releases {
			release.commits.reverse();
		}
	}

	// Add custom commit messages to the latest release.
	if let Some(custom_commits) = &args.with_commit {
		releases
			.last_mut()
			.unwrap()
			.commits
			.extend(custom_commits.iter().cloned().map(Commit::from));
	}

	// Set the previous release if the first release does not have one set.
	if releases[0]
		.previous
		.as_ref()
		.and_then(|p| p.version.as_ref())
		.is_none()
	{
		// Get the previous tag of the first processed tag in the release loop.
		let first_tag = first_processed_tag
			.map(|tag| {
				tags.iter()
					.enumerate()
					.find(|(_, (_, v))| v.name == tag.name)
					.and_then(|(i, _)| i.checked_sub(1))
					.and_then(|i| tags.get_index(i))
			})
			.or_else(|| Some(tags.last()))
			.flatten();

		// Set the previous release if the first tag is found.
		if let Some((commit_id, tag)) = first_tag {
			let previous_release = Release {
				commit_id: Some(commit_id.to_string()),
				version: Some(tag.name.clone()),
				timestamp: repository
					.find_commit(commit_id)
					.map(|v| v.time().seconds())
					.unwrap_or_default(),
				..Default::default()
			};
			releases[0].previous = Some(Box::new(previous_release));
		}
	}

	for release in &mut releases {
		// Set the commit ranges for all releases
		if !release.commits.is_empty() {
			release.commit_range = Some(match args.sort {
				Sort::Oldest => Range::new(
					release.commits.first().unwrap(),
					release.commits.last().unwrap(),
				),
				Sort::Newest => Range::new(
					release.commits.last().unwrap(),
					release.commits.first().unwrap(),
				),
			})
		}
		if recurse_submodules {
			process_submodules(repository, release, config.git.topo_order_commits)?;
		}
	}

	// Set custom message for the latest release.
	if let Some(message) = &args.with_tag_message {
		if let Some(latest_release) = releases
			.iter_mut()
			.filter(|release| !release.commits.is_empty())
			.next_back()
		{
			latest_release.message = Some(message.to_owned());
		}
	}

	Ok(releases)
}

/// Runs `git-cliff`.
///
/// # Example
///
/// ```no_run
/// use clap::Parser;
/// use git_cliff::args::Opt;
/// use git_cliff_core::error::Result;
///
/// fn main() -> Result<()> {
/// 	let args = Opt::parse();
/// 	git_cliff::run(args)?;
/// 	Ok(())
/// }
/// ```
pub fn run(args: Opt) -> Result<()> {
	run_with_changelog_modifier(args, |_| Ok(()))
}

/// Runs `git-cliff` with a changelog modifier.
///
/// This is useful if you want to modify the [`Changelog`] before
/// it's written or the context is printed (depending how git-cliff is started).
///
/// # Example
///
/// ```no_run
/// use clap::Parser;
/// use git_cliff::args::Opt;
/// use git_cliff_core::error::Result;
///
/// fn main() -> Result<()> {
/// 	let args = Opt::parse();
///
/// 	git_cliff::run_with_changelog_modifier(args, |changelog| {
/// 		println!("Releases: {:?}", changelog.releases);
/// 		Ok(())
/// 	})?;
///
/// 	Ok(())
/// }
/// ```
pub fn run_with_changelog_modifier(
	mut args: Opt,
	changelog_modifier: impl FnOnce(&mut Changelog) -> Result<()>,
) -> Result<()> {
	// Check if there is a new version available.
	#[cfg(feature = "update-informer")]
	check_new_version();

	// Create the configuration file if init flag is given.
	if let Some(init_config) = args.init {
		let contents = match init_config {
			Some(ref name) => BuiltinConfig::get_config(name.to_string())?,
			None => EmbeddedConfig::get_config()?,
		};

		let config_path = if args.config == PathBuf::from(DEFAULT_CONFIG) {
			PathBuf::from(DEFAULT_CONFIG)
		} else {
			args.config.clone()
		};

		info!(
			"Saving the configuration file{} to {:?}",
			init_config.map(|v| format!(" ({v})")).unwrap_or_default(),
			config_path
		);
		fs::write(config_path, contents)?;
		return Ok(());
	}

	// Retrieve the built-in configuration.
	let builtin_config =
		BuiltinConfig::parse(args.config.to_string_lossy().to_string());

	// Set the working directory.
	if let Some(ref workdir) = args.workdir {
		args.config = workdir.join(args.config);
		match args.repository.as_mut() {
			Some(repository) => {
				repository
					.iter_mut()
					.for_each(|r| *r = workdir.join(r.clone()));
			}
			None => args.repository = Some(vec![workdir.clone()]),
		}
		if let Some(changelog) = args.prepend {
			args.prepend = Some(workdir.join(changelog));
		}
	}

	// Set path for the configuration file.
	let mut path = args.config.clone();
	if !path.exists() {
		if let Some(config_path) = dirs::config_dir()
			.map(|dir| dir.join(env!("CARGO_PKG_NAME")).join(DEFAULT_CONFIG))
		{
			path = config_path;
		}
	}

	// Parse the configuration file.
	// Load the default configuration if necessary.
	let mut config = if let Some(url) = &args.config_url {
		debug!("Using configuration file from: {url}");
		#[cfg(feature = "remote")]
		{
			let contents = reqwest::blocking::get(url.clone())?
				.error_for_status()?
				.text()?;
			Config::parse_from_str(&contents)?
		}
		#[cfg(not(feature = "remote"))]
		unreachable!(
			"This option is not available without the 'remote' build-time feature"
		);
	} else if let Ok((config, name)) = builtin_config {
		info!("Using built-in configuration file: {name}");
		config
	} else if path.exists() {
		Config::parse(&path)?
	} else if let Some(contents) = Config::read_from_manifest()? {
		Config::parse_from_str(&contents)?
	} else if let Some(discovered_path) =
		env::current_dir()?.ancestors().find_map(|dir| {
			let path = dir.join(DEFAULT_CONFIG);
			if path.is_file() { Some(path) } else { None }
		}) {
		info!(
			"Using configuration from parent directory: {}",
			discovered_path.display()
		);
		Config::parse(&discovered_path)?
	} else {
		if !args.context {
			warn!(
				"{:?} is not found, using the default configuration.",
				args.config
			);
		}
		EmbeddedConfig::parse()?
	};

	// Update the configuration based on command line arguments and vice versa.
	let output = args.output.clone().or(config.changelog.output.clone());
	match args.strip {
		Some(Strip::Header) => {
			config.changelog.header = None;
		}
		Some(Strip::Footer) => {
			config.changelog.footer = None;
		}
		Some(Strip::All) => {
			config.changelog.header = None;
			config.changelog.footer = None;
		}
		None => {}
	}
	if args.prepend.is_some() {
		config.changelog.footer = None;
		if !(args.unreleased || args.latest || args.range.is_some()) {
			return Err(Error::ArgumentError(String::from(
				"'-u' or '-l' is not specified",
			)));
		}
	}
	if output.is_some() &&
		args.prepend.is_some() &&
		output.as_ref() == args.prepend.as_ref()
	{
		return Err(Error::ArgumentError(String::from(
			"'-o' and '-p' can only be used together if they point to different \
			 files",
		)));
	}
	if let Some(body) = args.body.clone() {
		config.changelog.body = body;
	}
	if args.sort == Sort::Oldest {
		args.sort = Sort::from_str(&config.git.sort_commits, true)
			.expect("Incorrect config value for 'sort_commits'");
	}
	if !args.topo_order {
		args.topo_order = config.git.topo_order;
	}

	if !args.use_branch_tags {
		args.use_branch_tags = config.git.use_branch_tags;
	}

	if args.github_token.is_some() {
		config.remote.github.token.clone_from(&args.github_token);
	}
	if args.gitlab_token.is_some() {
		config.remote.gitlab.token.clone_from(&args.gitlab_token);
	}
	if args.gitea_token.is_some() {
		config.remote.gitea.token.clone_from(&args.gitea_token);
	}
	if args.bitbucket_token.is_some() {
		config
			.remote
			.bitbucket
			.token
			.clone_from(&args.bitbucket_token);
	}
	if let Some(ref remote) = args.github_repo {
		config.remote.github.owner = remote.0.owner.to_string();
		config.remote.github.repo = remote.0.repo.to_string();
		config.remote.github.is_custom = true;
	}
	if let Some(ref remote) = args.gitlab_repo {
		config.remote.gitlab.owner = remote.0.owner.to_string();
		config.remote.gitlab.repo = remote.0.repo.to_string();
		config.remote.gitlab.is_custom = true;
	}
	if let Some(ref remote) = args.bitbucket_repo {
		config.remote.bitbucket.owner = remote.0.owner.to_string();
		config.remote.bitbucket.repo = remote.0.repo.to_string();
		config.remote.bitbucket.is_custom = true;
	}
	if let Some(ref remote) = args.gitea_repo {
		config.remote.gitea.owner = remote.0.owner.to_string();
		config.remote.gitea.repo = remote.0.repo.to_string();
		config.remote.gitea.is_custom = true;
	}
	if args.no_exec {
		config
			.git
			.commit_preprocessors
			.iter_mut()
			.for_each(|v| v.replace_command = None);
		config
			.changelog
			.postprocessors
			.iter_mut()
			.for_each(|v| v.replace_command = None);
	}
	config.git.skip_tags = config.git.skip_tags.filter(|r| !r.as_str().is_empty());
	if args.tag_pattern.is_some() {
		config.git.tag_pattern.clone_from(&args.tag_pattern);
	}
	if args.tag.is_some() {
		config.bump.initial_tag.clone_from(&args.tag);
	}
	if args.ignore_tags.is_some() {
		config.git.ignore_tags.clone_from(&args.ignore_tags);
	}
	if args.count_tags.is_some() {
		config.git.count_tags.clone_from(&args.count_tags);
	}

	// Process commits and releases for the changelog.
	if let Some(BumpOption::Specific(bump_type)) = args.bump {
		config.bump.bump_type = Some(bump_type);
	}

	// Generate changelog from context.
	let mut changelog: Changelog = if let Some(context_path) = args.from_context {
		let mut input: Box<dyn io::Read> = if context_path == Path::new("-") {
			Box::new(io::stdin())
		} else {
			Box::new(File::open(context_path)?)
		};
		let mut changelog = Changelog::from_context(&mut input, &config)?;
		changelog.add_remote_context()?;
		changelog
	} else {
		// Process the repositories.
		let repositories =
			args.repository.clone().unwrap_or(vec![env::current_dir()?]);
		let mut releases = Vec::<Release>::new();
		let mut commit_range = None;
		for repository in repositories {
			// Skip commits
			let mut skip_list = Vec::new();
			let ignore_file = repository.join(IGNORE_FILE);
			if ignore_file.exists() {
				let contents = fs::read_to_string(ignore_file)?;
				let commits = contents
					.lines()
					.filter(|v| !(v.starts_with('#') || v.trim().is_empty()))
					.map(|v| String::from(v.trim()))
					.collect::<Vec<String>>();
				skip_list.extend(commits);
			}
			if let Some(ref skip_commit) = args.skip_commit {
				skip_list.extend(skip_commit.clone());
			}
			for sha1 in skip_list {
				config.git.commit_parsers.insert(0, CommitParser {
					sha: Some(sha1.to_string()),
					skip: Some(true),
					..Default::default()
				});
			}

			// Process the repository.
			let repository = Repository::init(repository)?;

			// The commit range, used for determining the remote commits to include
			// in the changelog, doesn't make sense if multiple repositories are
			// specified. As such, pick the commit range from the last given
			// repository.
			commit_range = determine_commit_range(&args, &config, &repository)?;

			releases.extend(process_repository(
				Box::leak(Box::new(repository)),
				&mut config,
				&args,
			)?);
		}
		Changelog::new(releases, &config, commit_range.as_deref())?
	};
	changelog_modifier(&mut changelog)?;

	// Print the result.
	let mut out: Box<dyn io::Write> = if let Some(path) = &output {
		if path == Path::new("-") {
			Box::new(io::stdout())
		} else {
			Box::new(io::BufWriter::new(File::create(path)?))
		}
	} else {
		Box::new(io::stdout())
	};
	if args.bump.is_some() || args.bumped_version {
		let next_version = if let Some(next_version) = changelog.bump_version()? {
			next_version
		} else if let Some(last_version) =
			changelog.releases.first().cloned().and_then(|v| v.version)
		{
			warn!("There is nothing to bump.");
			last_version
		} else if changelog.releases.is_empty() {
			config.bump.get_initial_tag()
		} else {
			return Ok(());
		};
		if let Some(tag_pattern) = &config.git.tag_pattern {
			if !tag_pattern.is_match(&next_version) {
				return Err(Error::ChangelogError(format!(
					"Next version ({}) does not match the tag pattern: {}",
					next_version, tag_pattern
				)));
			}
		}
		if args.bumped_version {
			writeln!(out, "{next_version}")?;
			return Ok(());
		}
	}
	if args.context {
		changelog.write_context(&mut out)?;
		return Ok(());
	}
	if let Some(path) = &args.prepend {
		let changelog_before = fs::read_to_string(path)?;
		let mut out = io::BufWriter::new(File::create(path)?);
		changelog.prepend(changelog_before, &mut out)?;
	}
	if output.is_some() || args.prepend.is_none() {
		changelog.generate(&mut out)?;
	}

	Ok(())
}
