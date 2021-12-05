/// Command-line argument parser.
pub mod args;
/// Changelog generator.
pub mod changelog;

#[macro_use]
extern crate log;

use args::Opt;
use changelog::Changelog;
use git_cliff_core::commit::Commit;
use git_cliff_core::config::Config;
use git_cliff_core::embed::EmbeddedConfig;
use git_cliff_core::error::{
	Error,
	Result,
};
use git_cliff_core::release::Release;
use git_cliff_core::repo::Repository;
use git_cliff_core::DEFAULT_CONFIG;
use std::env;
use std::fs::{
	self,
	File,
};
use std::io;

/// Runs `git-cliff`.
pub fn run(mut args: Opt) -> Result<()> {
	// Create the configuration file if init flag is given.
	if args.init {
		info!("Saving the configuration file to {:?}", DEFAULT_CONFIG);
		fs::write(DEFAULT_CONFIG, EmbeddedConfig::get_config()?)?;
		return Ok(());
	}

	// Set the working directory.
	if let Some(workdir) = args.workdir {
		args.config = workdir.join(args.config);
		args.repository = match args.repository {
			Some(repository) => Some(workdir.join(repository)),
			None => Some(workdir.clone()),
		};
		if let Some(changelog) = args.prepend {
			args.prepend = Some(workdir.join(changelog));
		}
	}

	// Parse the configuration file.
	let mut path = match args.config.to_str() {
		Some(v) => Ok(v.to_string()),
		None => Err(Error::IoError(io::Error::new(
			io::ErrorKind::Other,
			"path contains invalid characters",
		))),
	}?;
	if let Some(config_path) = dirs_next::config_dir()
		.map(|dir| dir.join(env!("CARGO_PKG_NAME")).join(DEFAULT_CONFIG))
		.map(|path| path.to_str().map(String::from))
		.flatten()
	{
		if fs::metadata(&config_path).is_ok() {
			path = config_path;
		}
	}

	let mut config = if fs::metadata(&path).is_ok() {
		Config::parse(path)?
	} else {
		warn!("{:?} is not found, using the default configuration.", path);
		EmbeddedConfig::parse()?
	};

	// Update the configuration based on command line arguments and vice versa.
	match args.strip.as_deref() {
		Some("header") => {
			config.changelog.header = None;
		}
		Some("footer") => {
			config.changelog.footer = None;
		}
		Some("all") => {
			config.changelog.header = None;
			config.changelog.footer = None;
		}
		_ => {}
	}
	if args.prepend.is_some() {
		config.changelog.footer = None;
		if !(args.unreleased || args.latest) {
			return Err(Error::ArgumentError(String::from(
				"'-u' or '-l' is not specified",
			)));
		}
	}
	if let Some(template) = args.body {
		config.changelog.body = template;
	}
	if args.sort == "oldest" {
		if let Some(ref sort_commits) = config.git.sort_commits {
			args.sort = sort_commits.to_string();
		}
	}
	if !args.topo_order {
		if let Some(topo_order) = config.git.topo_order {
			args.topo_order = topo_order;
		}
	}

	// Initialize the git repository.
	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;

	// Parse tags.
	let mut tags = repository.tags(&config.git.tag_pattern, args.topo_order)?;

	// Skip tags.
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

	// Parse commits.
	let mut commit_range = args.range;
	if args.unreleased {
		if let Some(last_tag) = tags.last().map(|(k, _)| k) {
			commit_range = Some(format!("{}..HEAD", last_tag));
		}
	} else if args.latest || args.current {
		if tags.len() < 2 {
			return Err(Error::ChangelogError(String::from(
				"Not enough tags exist for processing the latest/current tag",
			)));
		}
		let mut tag_index = tags.len() - 2;
		if args.current {
			if let Some(current_tag_index) =
				repository.current_tag().as_ref().and_then(|tag| {
					tags.iter()
						.enumerate()
						.find(|(_, (_, v))| v == &tag)
						.map(|(i, _)| i)
				}) {
				tag_index = current_tag_index - 1;
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
			commit_range = Some(format!("{}..{}", tag1, tag2));
		}
	}
	let commits =
		repository.commits(commit_range, args.include_path, args.exclude_path)?;

	// Update tags.
	if let Some(tag) = args.tag {
		if let Some(commit_id) = commits.first().map(|c| c.id().to_string()) {
			match tags.get(&commit_id) {
				Some(tag) => {
					warn!("There is already a tag ({}) for {}", tag, commit_id)
				}
				None => {
					tags.insert(commit_id, tag);
				}
			}
		}
	}

	// Process releases.
	let mut releases = vec![Release::default()];
	let mut release_index = 0;
	let mut previous_release = Release::default();
	for git_commit in commits.into_iter().rev() {
		let commit = Commit::from(&git_commit);
		let commit_id = commit.id.to_string();
		if args.sort == "newest" {
			releases[release_index].commits.insert(0, commit);
		} else {
			releases[release_index].commits.push(commit);
		}
		if let Some(tag) = tags.get(&commit_id) {
			releases[release_index].version = Some(tag.to_string());
			releases[release_index].commit_id = Some(commit_id);
			releases[release_index].timestamp = git_commit.time().seconds();
			previous_release.previous = None;
			releases[release_index].previous = Some(Box::new(previous_release));
			previous_release = releases[release_index].clone();
			releases.push(Release::default());
			release_index += 1;
		}
	}

	// Set the previous release if needed.
	if args.latest {
		if let Some((commit_id, version)) = tags.get_index(tags.len() - 2) {
			let previous_release = Release {
				commit_id: Some(commit_id.to_string()),
				version: Some(version.to_string()),
				..Release::default()
			};
			releases[0].previous = Some(Box::new(previous_release));
		}
	}

	// Generate changelog.
	let changelog = Changelog::new(releases, &config)?;
	if let Some(path) = args.prepend {
		changelog.prepend(fs::read_to_string(&path)?, &mut File::create(path)?)
	} else if let Some(path) = args.output {
		changelog.generate(&mut File::create(path)?)
	} else {
		changelog.generate(&mut io::stdout())
	}
}
