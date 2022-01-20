use clap::Parser;
use git_cliff::args::Opt;
use std::env;
use std::process;

fn main() {
	let args = Opt::parse();
	if args.verbose == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbose > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	pretty_env_logger::init();
	match git_cliff::run(args) {
		Ok(_) => process::exit(0),
		Err(e) => {
			log::error!("{}", e);
			process::exit(1)
		}
	}
}
