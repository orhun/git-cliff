use clap::Parser;
use git_cliff::args::Opt;
use git_cliff_core::error::Result;

fn main() -> Result<()> {
    let args = Opt::parse();
    let changelog = git_cliff::run(args.clone())?;
    git_cliff::write_changelog(&args, changelog, std::io::stdout())?;
    Ok(())
}
