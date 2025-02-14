use clap::Parser;
use git_cliff::args::Args;
use git_cliff_core::error::Result;

fn main() -> Result<()> {
	let args = Args::parse();
	git_cliff::run(args)?;
	Ok(())
}
