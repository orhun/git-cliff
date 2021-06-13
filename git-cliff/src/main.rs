use git_cliff::args::Opt;
use std::env;
use std::process;
use structopt::StructOpt;

fn main() {
	let args = Opt::from_args();
	if args.debug {
		env::set_var("RUST_LOG", "debug");
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
