use std::fs::File;
use std::path::Path;
use std::{env, io, process};

use clap::Parser;
use git_cliff::args::Opt;
use git_cliff::{init_config, logger};
use git_cliff_core::error::Result;

/// Profiler.
#[cfg(feature = "profiler")]
mod profiler;

fn main() -> Result<()> {
    // Parse the command line arguments
    let args = Opt::parse();
    if args.verbose == 1 {
        unsafe { env::set_var("RUST_LOG", "debug") };
    } else if args.verbose > 1 {
        unsafe { env::set_var("RUST_LOG", "trace") };
    } else if env::var_os("RUST_LOG").is_none() {
        unsafe { env::set_var("RUST_LOG", "info") };
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

    // Check if there is a new version available.
    #[cfg(feature = "update-informer")]
    if !args.offline {
        git_cliff::check_new_version();
    }

    // Create the configuration file if init flag is given.
    if let Some(path) = &args.init {
        init_config(path.as_deref(), &args.config)?;
        return Ok(());
    }

    // Generate a changelog.
    let changelog = git_cliff::run(args.clone())?;

    // Get output destination.
    let output = args
        .output
        .clone()
        .or(changelog.config.changelog.output.clone());
    let out: Box<dyn io::Write> = if let Some(path) = &output {
        if path == Path::new("-") {
            Box::new(io::stdout())
        } else {
            Box::new(io::BufWriter::new(File::create(path)?))
        }
    } else {
        Box::new(io::stdout())
    };

    // Write the changelog.
    let exit_code = match git_cliff::write_changelog(&args, changelog, out) {
        Ok(()) => 0,
        Err(e) => {
            log::error!("{e}");
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
