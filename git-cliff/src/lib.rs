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
	Opt,
	Sort,
	Strip,
};
use clap::ValueEnum;
use git_cliff_core::changelog::Changelog;
use git_cliff_core::commit::Commit;
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
use git_cliff_core::repo::Repository;
use git_cliff_core::{
	DEFAULT_CONFIG,
	IGNORE_FILE,
};
use std::env;
use std::fs::{
	self,
	File,
};
use std::io::{
	self,
	Write,
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
	let mut tags = repository.tags(&config.git.tag_pattern, args.topo_order)?;
	let skip_regex = config.git.skip_tags.as_ref();
	let ignore_regex = config.git.ignore_tags.as_ref();
	tags = tags
		.into_iter()
		.filter(|(_, name)| {
			// Keep skip tags to drop commits in the later stage.
			let skip = skip_regex.map(|r| r.is_match(name)).unwrap_or_default();

			let ignore = ignore_regex
				.map(|r| {
					if r.as_str().trim().is_empty() {
						return false;
					}

					let ignore_tag = r.is_match(name);
					if ignore_tag {
						trace!("Ignoring release: {}", name)
					}
					ignore_tag
				})
				.unwrap_or_default();

			skip || !ignore
		})
		.collect();

	if !config.remote.github.is_set() {
		match repository.upstream_remote() {
			Ok(remote) => {
				debug!("No GitHub remote is set, using remote: {}", remote);
				config.remote.github.owner = remote.owner;
				config.remote.github.repo = remote.repo;
			}
			Err(e) => {
				debug!("Failed to get remote from repository: {:?}", e);
			}
		}
	}

	// Print debug information about configuration and arguments.
	log::trace!("Arguments: {:#?}", args);
	log::trace!("Config: {:#?}", config);

	// Parse commits.
	let mut commit_range = args.range.clone();
	if args.unreleased {
		if let Some(last_tag) = tags.last().map(|(k, _)| k) {
			commit_range = Some(format!("{last_tag}..HEAD"));
		}
	} else if args.latest || args.current {
		if tags.len() < 2 {
			let commits = repository.commits(None, None, None)?;
			if let (Some(tag1), Some(tag2)) = (
				commits.last().map(|c| c.id().to_string()),
				tags.get_index(0).map(|(k, _)| k),
			) {
				commit_range = Some(format!("{tag1}..{tag2}"));
			}
		} else {
			let mut tag_index = tags.len() - 2;
			if args.current {
				if let Some(current_tag_index) =
					repository.current_tag().as_ref().and_then(|tag| {
						tags.iter()
							.enumerate()
							.find(|(_, (_, v))| v == &tag)
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
	let mut commits = repository.commits(
		commit_range,
		args.include_path.clone(),
		args.exclude_path.clone(),
	)?;
	if let Some(commit_limit_value) = config.git.limit_commits {
		commits = commits
			.drain(..commits.len().min(commit_limit_value))
			.collect();
	}

	// Update tags.
	if let Some(ref tag) = args.tag {
		if let Some(commit_id) = commits.first().map(|c| c.id().to_string()) {
			match tags.get(&commit_id) {
				Some(tag) => {
					warn!("There is already a tag ({}) for {}", tag, commit_id)
				}
				None => {
					tags.insert(commit_id, tag.to_string());
				}
			}
		}
	}

	// Process releases.
	let mut releases = vec![Release::default()];
	let mut release_index = 0;
	let mut previous_release = Release::default();
	let mut first_processed_tag = None;
	for git_commit in commits.iter().rev() {
		let commit = Commit::from(git_commit);
		let commit_id = commit.id.to_string();
		if args.sort == Sort::Newest {
			releases[release_index].commits.insert(0, commit);
		} else {
			releases[release_index].commits.push(commit);
		}
		if let Some(tag) = tags.get(&commit_id) {
			releases[release_index].version = Some(tag.to_string());
			releases[release_index].commit_id = Some(commit_id);
			releases[release_index].timestamp = if args.tag.as_deref() == Some(tag) {
				SystemTime::now()
					.duration_since(UNIX_EPOCH)?
					.as_secs()
					.try_into()?
			} else {
				git_commit.time().seconds()
			};
			if first_processed_tag.is_none() {
				first_processed_tag = Some(tag);
			}
			previous_release.previous = None;
			releases[release_index].previous = Some(Box::new(previous_release));
			previous_release = releases[release_index].clone();
			releases.push(Release::default());
			release_index += 1;
		}
	}

	if release_index > 0 {
		previous_release.previous = None;
		releases[release_index].previous = Some(Box::new(previous_release));
	}

	// Add custom commit messages to the latest release.
	if let Some(custom_commits) = &args.with_commit {
		if let Some(latest_release) = releases.iter_mut().last() {
			custom_commits.iter().for_each(|message| {
				latest_release
					.commits
					.push(Commit::from(message.to_string()))
			});
		}
	}

	// Set the previous release if the first release does not have one set.
	if !releases.is_empty() &&
		releases
			.first()
			.and_then(|r| r.previous.as_ref())
			.and_then(|p| p.version.as_ref())
			.is_none()
	{
		// Get the previous tag of the first processed tag in the release loop.
		let first_tag = first_processed_tag
			.map(|tag| {
				tags.iter()
					.enumerate()
					.find(|(_, (_, v))| v == &tag)
					.and_then(|(i, _)| i.checked_sub(1))
					.and_then(|i| tags.get_index(i))
			})
			.or_else(|| Some(tags.last()))
			.flatten();

		// Set the previous release if the first tag is found.
		if let Some((commit_id, version)) = first_tag {
			let previous_release = Release {
				commit_id: Some(commit_id.to_string()),
				version: Some(version.to_string()),
				timestamp: repository
					.find_commit(commit_id.to_string())
					.map(|v| v.time().seconds())
					.unwrap_or_default(),
				..Default::default()
			};
			releases[0].previous = Some(Box::new(previous_release));
		}
	}

	Ok(releases)
}

/// Runs `git-cliff`.
pub fn run(mut args: Opt) -> Result<()> {
	// Check if there is a new version available.
	#[cfg(feature = "update-informer")]
	check_new_version();

	// Create the configuration file if init flag is given.
	if let Some(init_config) = args.init {
		let contents = match init_config {
			Some(ref name) => BuiltinConfig::get_config(name.to_string())?,
			None => EmbeddedConfig::get_config()?,
		};
		info!(
			"Saving the configuration file{} to {:?}",
			init_config.map(|v| format!(" ({v})")).unwrap_or_default(),
			DEFAULT_CONFIG
		);
		fs::write(DEFAULT_CONFIG, contents)?;
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

	// Parse the configuration file.
	let mut path = args.config.clone();
	if !path.exists() {
		if let Some(config_path) = dirs::config_dir()
			.map(|dir| dir.join(env!("CARGO_PKG_NAME")).join(DEFAULT_CONFIG))
		{
			path = config_path;
		}
	}

	// Load the default configuration if necessary.
	let mut config = if let Ok((config, name)) = builtin_config {
		info!("Using built-in configuration file: {name}");
		config
	} else if path.exists() {
		Config::parse(&path)?
	} else if let Some(contents) = Config::read_from_manifest()? {
		Config::parse_from_str(&contents)?
	} else {
		if !args.context {
			warn!(
				"{:?} is not found, using the default configuration.",
				args.config
			);
		}
		EmbeddedConfig::parse()?
	};
	if config.changelog.body.is_none() && !args.context {
		warn!("Changelog body is not specified, using the default template.");
		config.changelog.body = EmbeddedConfig::parse()?.changelog.body;
	}

	// Update the configuration based on command line arguments and vice versa.
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
	if args.body.is_some() {
		config.changelog.body.clone_from(&args.body);
	}
	if args.sort == Sort::Oldest {
		if let Some(ref sort_commits) = config.git.sort_commits {
			args.sort = Sort::from_str(sort_commits, true)
				.expect("Incorrect config value for 'sort_commits'");
		}
	}
	if !args.topo_order {
		if let Some(topo_order) = config.git.topo_order {
			args.topo_order = topo_order;
		}
	}
	if args.github_token.is_some() {
		config.remote.github.token.clone_from(&args.github_token);
	}
	if let Some(ref remote) = args.github_repo {
		config.remote.github.owner = remote.0.owner.to_string();
		config.remote.github.repo = remote.0.repo.to_string();
	}
	if args.no_exec {
		if let Some(ref mut preprocessors) = config.git.commit_preprocessors {
			preprocessors
				.iter_mut()
				.for_each(|v| v.replace_command = None);
		}
		if let Some(ref mut postprocessors) = config.changelog.postprocessors {
			postprocessors
				.iter_mut()
				.for_each(|v| v.replace_command = None);
		}
	}
	config.git.skip_tags = config.git.skip_tags.filter(|r| !r.as_str().is_empty());
	if args.tag_pattern.is_some() {
		config.git.tag_pattern.clone_from(&args.tag_pattern);
	}

	// Process the repositories.
	let repositories = args.repository.clone().unwrap_or(vec![env::current_dir()?]);
	let mut releases = Vec::<Release>::new();
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
		if let Some(commit_parsers) = config.git.commit_parsers.as_mut() {
			for sha1 in skip_list {
				commit_parsers.insert(0, CommitParser {
					sha: Some(sha1.to_string()),
					skip: Some(true),
					..Default::default()
				})
			}
		}

		// Process the repository.
		let repository = Repository::init(repository)?;
		releases.extend(process_repository(
			Box::leak(Box::new(repository)),
			&mut config,
			&args,
		)?);
	}

	// Process commits and releases for the changelog.
	let mut changelog = Changelog::new(releases, &config)?;

	// Print the result.
	if args.bump || args.bumped_version {
		let next_version = if let Some(next_version) = changelog.bump_version()? {
			next_version
		} else if let Some(last_version) =
			changelog.releases.first().cloned().and_then(|v| v.version)
		{
			warn!("There is nothing to bump.");
			last_version
		} else {
			return Ok(());
		};
		if args.bumped_version {
			if let Some(path) = args.output {
				let mut output = File::create(path)?;
				output.write_all(next_version.as_bytes())?;
			} else {
				println!("{next_version}");
			}
			return Ok(());
		}
	}
	if args.context {
		return if let Some(path) = args.output {
			let mut output = File::create(path)?;
			changelog.write_context(&mut output)
		} else {
			changelog.write_context(&mut io::stdout())
		};
	}
	if let Some(ref path) = args.prepend {
		changelog.prepend(fs::read_to_string(path)?, &mut File::create(path)?)?;
	}
	if let Some(path) = args.output {
		let mut output = File::create(path)?;
		if args.context {
			changelog.write_context(&mut output)
		} else {
			changelog.generate(&mut output)
		}
	} else if args.prepend.is_none() {
		changelog.generate(&mut io::stdout())
	} else {
		Ok(())
	}
}
