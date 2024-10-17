use clap::Parser;
use git_cliff::args::Args;
use git_cliff::logger;
use git_cliff_core::error::Result;
use std::env;
use std::fs::File;
use std::io::{
	self,
};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{
	self,
	Command,
	Stdio,
};

/// Profiler.
#[cfg(feature = "profiler")]
mod profiler;

fn main() -> Result<()> {
	// Parse the command line arguments
	let args = Args::parse();

	// Launch the TUI if the flag is set.
	if args.tui {
		let env_args = env::args().collect::<Vec<String>>();
		let error = Command::new("git-cliff-tui")
			.stdout(Stdio::inherit())
			.args(
				&env_args[1..]
					.iter()
					.filter(|arg| *arg != "--tui")
					.collect::<Vec<&String>>(),
			)
			.exec();
		return Err(error.into());
	}

	// Enable logging.
	if args.verbose == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbose > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	logger::init()?;

	// Initialize the profiler guard if the feature is enabled
	let mut _profiler_guard = None;
	#[cfg(feature = "profiler")]
	{
		_profiler_guard = profiler::start_profiling();
	}
	#[cfg(not(feature = "profiler"))]
	{
		_profiler_guard = Some(());
	}

	// Run git-cliff
	let out: Box<dyn io::Write> = if let Some(path) = &args.output {
		if path == Path::new("-") {
			Box::new(io::stdout())
		} else {
			Box::new(io::BufWriter::new(File::create(path)?))
		}
	} else {
		Box::new(io::stdout())
	};
	let exit_code = match git_cliff::run(args, out) {
		Ok(_) => 0,
		Err(e) => {
			log::error!("{}", e);
			1
		}
	};

	// Report the profiler if the feature is enabled
	#[cfg(feature = "profiler")]
	{
		profiler::finish_profiling(_profiler_guard)?;
	}

	process::exit(exit_code);
}
