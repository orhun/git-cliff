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
use git_cliff_core::error::{
	Error,
	Result,
};
use git_cliff_core::release::Release;
use git_cliff_core::repo::Repository;
use std::env;
use std::fs::{
	self,
	File,
};
use std::io;

/// Runs `git-cliff`.
pub fn run(mut args: Opt) -> Result<()> {
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

	// Parse configuration file.
	let mut config = Config::parse(match args.config.to_str() {
		Some(v) => Ok(v.to_string()),
		None => Err(Error::IoError(io::Error::new(
			io::ErrorKind::Other,
			"path contains invalid characters",
		))),
	}?)?;

	// Update the configuration based on command line arguments.
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

	// Initialize the git repository.
	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;

	// Parse tags.
	let mut tags = repository.tags(&config.git.tag_pattern)?;

	// Parse commits.
	let mut commit_range = args.range;
	if args.unreleased {
		if let Some(last_tag) = tags.last().map(|(k, _)| k) {
			commit_range = Some(format!("{}..HEAD", last_tag));
		}
	} else if args.latest {
		if tags.len() < 2 {
			return Err(Error::ChangelogError(String::from(
				"latest tag cannot be processed",
			)));
		} else if let (Some(tag1), Some(tag2)) = (
			tags.get_index(tags.len() - 2).map(|(k, _)| k),
			tags.get_index(tags.len() - 1).map(|(k, _)| k),
		) {
			commit_range = Some(format!("{}..{}", tag1, tag2));
		}
	}
	let commits = repository.commits(commit_range)?;

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
	let mut releases = vec![Release::default(); tags.len() + 1];
	let mut release_index = 0;
	let mut previous_release = Release::default();
	for git_commit in commits.into_iter().rev() {
		let commit = Commit::from(&git_commit);
		let commit_id = commit.id.to_string();
		releases[release_index].commits.push(commit);
		if let Some(tag) = tags.get(&commit_id) {
			releases[release_index].version = Some(tag.to_string());
			releases[release_index].commit_id = Some(commit_id);
			releases[release_index].timestamp = git_commit.time().seconds();
			previous_release.previous = None;
			releases[release_index].previous = Some(Box::new(previous_release));
			previous_release = releases[release_index].clone();
			release_index += 1;
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
