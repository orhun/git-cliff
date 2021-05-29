mod args;

use args::Opt;
use gitolith_core::changelog::Changelog;
use gitolith_core::commit::Commit;
use gitolith_core::config::Config;
use gitolith_core::error::Result;
use gitolith_core::release::{
	Release,
	ReleaseRoot,
};
use gitolith_core::repo::Repository;
use std::env;
use std::io::{
	self,
	Write,
};
use structopt::StructOpt;
#[macro_use]
extern crate log;

fn main() -> Result<()> {
	let args = Opt::from_args();
	if args.debug {
		env::set_var("RUST_LOG", "debug");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	pretty_env_logger::init();
	let config = Config::parse(args.config)?;

	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;
	let mut tags = repository.tags(&config.changelog.git_tag_pattern)?;
	let commits = repository.commits()?;

	if let Some(tag) = args.tag {
		if let Some(commit_id) = commits.first().map(|c| c.id().to_string()) {
			match tags.get(&commit_id) {
				Some(tag) => {
					debug!("There is already a tag ({}) for {}", tag, commit_id)
				}
				None => {
					tags.insert(commit_id, tag);
				}
			}
		}
	}

	let mut release_root = ReleaseRoot {
		releases: vec![Release::default(); tags.len() + 1],
	};
	let mut release_index = 0;
	for git_commit in commits.into_iter().rev() {
		let commit = Commit::from(&git_commit);
		let commit_id = commit.id.to_string();
		release_root.releases[release_index].commits.push(commit);
		if let Some(tag) = tags.get(&commit_id) {
			release_root.releases[release_index].version = Some(tag.to_string());
			release_root.releases[release_index].commit_id = Some(commit_id);
			release_root.releases[release_index].timestamp =
				git_commit.time().seconds();
			release_index += 1;
		}
	}

	release_root.releases.iter_mut().for_each(|release| {
		release.commits = release
			.commits
			.iter()
			.filter_map(|commit| {
				match commit.process(
					&config.changelog.commit_parsers,
					config.changelog.filter_group,
				) {
					Ok(commit) => Some(commit),
					Err(e) => {
						debug!("Cannot process commit: {} ({})", commit.id, e);
						None
					}
				}
			})
			.collect::<Vec<Commit>>();
	});
	release_root.releases = release_root
		.releases
		.into_iter()
		.rev()
		.filter(|release| {
			if release.commits.is_empty() {
				debug!(
					"Release {} doesn't have any commits",
					release
						.version
						.as_ref()
						.cloned()
						.unwrap_or_else(|| String::from("[?]"))
				);
				false
			} else if let Some(version) = &release.version {
				!config.changelog.skip_tags_regex.is_match(version)
			} else {
				true
			}
		})
		.collect();

	let stdout = &mut io::stdout();
	let changelog = Changelog::new(config.changelog.body)?;
	if !config.changelog.header.is_empty() {
		writeln!(stdout, "{}", config.changelog.header)?;
	}
	for release in release_root.releases {
		write!(stdout, "{}", changelog.generate(release)?)?;
	}
	if !config.changelog.footer.is_empty() {
		writeln!(stdout, "{}", config.changelog.footer)?;
	}

	Ok(())
}
