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

	let mut release_root = ReleaseRoot {
		releases: vec![Release::default(), Release::default()],
	};
	let repository =
		Repository::init(args.repository.unwrap_or(env::current_dir()?))?;
	for git_commit in repository.commits()? {
		release_root.releases[0]
			.commits
			.push(Commit::from(git_commit));
	}
	release_root.releases[1].version = Some(String::from("v1.0.0"));
	release_root.releases[1].commits.extend(vec![
		Commit::new(String::from("abc123"), String::from("feat: add xyz")),
		Commit::new(String::from("def789"), String::from("invalid commit")),
	]);

	release_root.releases.iter_mut().for_each(|release| {
		release.commits = release
			.commits
			.iter()
			.filter_map(|commit| match commit.as_conventional() {
				Ok(mut commit) => {
					commit.set_group(&config.changelog.group_parsers);
					Some(commit)
				}
				Err(e) => {
					debug!("{} is not conventional: {}", commit.id, e);
					None
				}
			})
			.collect::<Vec<Commit>>();
	});

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
