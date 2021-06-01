mod args;
mod changelog;

use args::Opt;
use changelog::Changelog;
use git_cliff_core::commit::Commit;
use git_cliff_core::config::Config;
use git_cliff_core::error::Result;
use git_cliff_core::release::Release;
use git_cliff_core::repo::Repository;
use std::env;
use std::io;
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

	let mut releases = vec![Release::default(); tags.len() + 1];
	let mut release_index = 0;
	for git_commit in commits.into_iter().rev() {
		let commit = Commit::from(&git_commit);
		let commit_id = commit.id.to_string();
		releases[release_index].commits.push(commit);
		if let Some(tag) = tags.get(&commit_id) {
			releases[release_index].version = Some(tag.to_string());
			releases[release_index].commit_id = Some(commit_id);
			releases[release_index].timestamp = git_commit.time().seconds();
			release_index += 1;
		}
	}

	Changelog::new(releases, &config.changelog)?.generate(&mut io::stdout())
}
