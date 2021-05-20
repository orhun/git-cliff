mod args;

use args::Opt;
use gitolith_core::changelog::generator::Changelog;
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

	let mut release_root = ReleaseRoot {
		releases: vec![Release::default()],
	};
	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;
	let commits = repository.commits()?;
	for commit in &commits {
		match commit.as_conventional() {
			Ok(_conv_commit) => release_root.releases[0]
				.commits
				.push(commit.short_id.to_string()),
			Err(e) => debug!("{} is not conventional: {}", commit.short_id, e),
		}
	}

	let changelog = Changelog::new(args.template, &config.changelog)?;
	for mut release in release_root.releases {
		release.commits = release
			.commits
			.iter()
			.filter_map(|v| commits.iter().find(|c| &c.short_id == v))
			.map(|commit| commit.message.to_string())
			.collect();
		writeln!(&mut io::stdout(), "{}", changelog.generate(release)?)?;
	}

	Ok(())
}
