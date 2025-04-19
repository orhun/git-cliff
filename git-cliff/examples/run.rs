use clap::Parser;
use git_cliff::args::Opt;
use git_cliff_core::error::Result;

fn main() -> Result<()> {
	let args = Opt::parse();
	git_cliff::run(args)?;
	Ok(())
}
