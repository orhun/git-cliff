use clap::Parser;
use git_cliff::args::Opt;
use git_cliff::args::SubCommands;
use git_cliff::logger;
use git_cliff_core::error::Result;
use std::env;
use std::process;

fn main() -> Result<()> {
	// parse command line arguments
	let args: Opt = Opt::parse();

	// set log level
	if args.verbose == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbose > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	logger::init()?;

	// run the command or subcommand
	let result = match &args.subcommand {
		Some(sub_command) => match sub_command {
			SubCommands::MigrateConfig(migrate_args) => {
				git_cliff_core::config::migrate::run(migrate_args)
			}
		},
		None => git_cliff::run(args),
	};

	match result {
		Ok(_) => process::exit(0),
		Err(e) => {
			log::error!("{}", e);
			process::exit(1)
		}
	}
}
