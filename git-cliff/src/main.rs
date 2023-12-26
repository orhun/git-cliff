use clap::Parser;
use git_cliff::args::Opt;
use git_cliff::logger;
use git_cliff_core::error::Result;
use std::env;
use std::process;

fn main() -> Result<()> {
	let args = Opt::parse();
	if args.verbose == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbose > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	logger::init()?;
	match git_cliff::run(args) {
		Ok(_) => process::exit(0),
		Err(e) => {
			log::error!("{}", e);
			process::exit(1)
		}
	}
}
