mod args;

use args::Opt;
use gitolith_core::error::Result;
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
	info!("hello world");
	debug!("debugging");

	Ok(())
}
