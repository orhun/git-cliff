mod args;

use args::Opt;
use gitolith_core::config::Config;
use gitolith_core::error::Result;
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
	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;
	for commit in repository.commits()? {
		match commit.as_conventional() {
			Ok(_conv_commit) => {
				info!("{:?} is conventional!", commit.hash)
			}
			Err(e) => warn!("{:?} is not conventional: {}", commit.hash, e),
		}
	}
	Ok(())
}
