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
	let tags = repository.tags(&config.changelog.tag_regex)?;
	let commits = repository.commits()?;

	let mut release_root = ReleaseRoot {
		releases: vec![Release::default(); tags.len() + 1],
	};
	let mut release_index = 0;
	for git_commit in commits.into_iter().rev() {
		let commit = Commit::from(git_commit);
		let commit_id = commit.id.to_string();
		release_root.releases[release_index].commits.push(commit);
		if let Some(tag) = tags.get(&commit_id) {
			release_root.releases[release_index].version = Some(tag.to_string());
			release_root.releases[release_index].commit_id = Some(commit_id);
			release_index += 1;
		}
	}

	release_root.releases.iter_mut().for_each(|release| {
		release.commits = release
			.commits
			.iter()
			.filter_map(|commit| {
				match commit.process(
					&config.changelog.group_parsers,
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
			let empty = release.commits.is_empty();
			if empty {
				debug!(
					"Release {} doesn't have any commits",
					release
						.version
						.as_ref()
						.cloned()
						.unwrap_or_else(|| String::from("[?]"))
				)
			}
			!empty
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
