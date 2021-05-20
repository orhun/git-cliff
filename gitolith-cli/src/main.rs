mod args;

use args::Opt;
use gitolith_core::config::Config;
use gitolith_core::error::Result;
use gitolith_core::release::{
	Release,
	ReleaseRoot,
};
use gitolith_core::repo::Repository;
use std::env;
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
	let _config = Config::parse(args.config)?;

	let mut release_root = ReleaseRoot {
		releases: vec![Release::default()],
	};
	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;
	for commit in repository.commits()? {
		match commit.as_conventional() {
			Ok(_conv_commit) => {
				release_root.releases[0].commits.push(commit.short_id)
			}
			Err(e) => warn!("{} is not conventional: {}", commit.short_id, e),
		}
	}

	Ok(())
}
