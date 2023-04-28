use clap::{
	ArgAction,
	Parser,
	ValueEnum,
};
use git_cliff_core::DEFAULT_CONFIG;
use glob::Pattern;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Strip {
	Header,
	Footer,
	All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Sort {
	Oldest,
	Newest,
}

/// Command-line arguments to parse.
#[derive(Debug, Parser)]
#[command(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
	help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
    override_usage = "git-cliff [FLAGS] [OPTIONS] [--] [RANGE]",
    next_help_heading = Some("OPTIONS"),
	disable_help_flag = true,
	disable_version_flag = true,
)]
pub struct Opt {
	#[arg(
		short,
		long,
		action = ArgAction::Help,
		global = true,
		help = "Prints help information",
		help_heading = "FLAGS"
	)]
	pub help:         Option<bool>,
	#[arg(
		short = 'V',
		long,
		action = ArgAction::Version,
		global = true,
		help = "Prints version information",
		help_heading = "FLAGS"
	)]
	pub version:      Option<bool>,
	/// Increases the logging verbosity.
	#[arg(short, long, action = ArgAction::Count, alias = "debug", help_heading = Some("FLAGS"))]
	pub verbose:      u8,
	/// Sets the configuration file.
	#[arg(short, long, env = "GIT_CLIFF_CONFIG", value_name = "PATH", default_value = DEFAULT_CONFIG)]
	pub config:       PathBuf,
	/// Sets the working directory.
	#[arg(short, long, env = "GIT_CLIFF_WORKDIR", value_name = "PATH")]
	pub workdir:      Option<PathBuf>,
	/// Sets the git repository.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_REPOSITORY",
		value_name = "PATH",
		num_args(1..)
	)]
	pub repository:   Option<Vec<PathBuf>>,
	/// Sets the path to include related commits.
	#[arg(
		long,
		env = "GIT_CLIFF_INCLUDE_PATH",
		value_name = "PATTERN",
		num_args(1..)
	)]
	pub include_path: Option<Vec<Pattern>>,
	/// Sets the path to exclude related commits.
	#[arg(
		long,
		env = "GIT_CLIFF_EXCLUDE_PATH",
		value_name = "PATTERN",
		num_args(1..)
	)]
	pub exclude_path: Option<Vec<Pattern>>,
	/// Sets custom commit messages to include in the changelog.
	#[arg(
		long,
		env = "GIT_CLIFF_WITH_COMMIT",
		value_name = "MSG",
		num_args(1..)
	)]
	pub with_commit:  Option<Vec<String>>,
	/// Prepends entries to the given changelog file.
	#[arg(short, long, env = "GIT_CLIFF_PREPEND", value_name = "PATH")]
	pub prepend:      Option<PathBuf>,
	/// Writes output to the given file.
	#[arg(short, long, env = "GIT_CLIFF_OUTPUT", value_name = "PATH")]
	pub output:       Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_TAG",
		value_name = "TAG",
		allow_hyphen_values = true
	)]
	pub tag:          Option<String>,
	/// Sets the template for the changelog body.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_TEMPLATE",
		value_name = "TEMPLATE",
		allow_hyphen_values = true
	)]
	pub body:         Option<String>,
	/// Writes the default configuration file to cliff.toml
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub init:         bool,
	/// Processes the commits starting from the latest tag.
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub latest:       bool,
	/// Processes the commits that belong to the current tag.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub current:      bool,
	/// Processes the commits that do not belong to a tag.
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub unreleased:   bool,
	/// Sorts the tags topologically.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub topo_order:   bool,
	/// Prints changelog context as JSON.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub context:      bool,
	/// Strips the given parts from the changelog.
	#[arg(short, long, value_name = "PART", value_enum)]
	pub strip:        Option<Strip>,
	/// Sets sorting of the commits inside sections.
	#[arg(
		long,
		value_enum,
		default_value_t = Sort::Oldest
	)]
	pub sort:         Sort,
	/// Sets the commit range to process.
	#[arg(value_name = "RANGE", help_heading = Some("ARGS"))]
	pub range:        Option<String>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;

	#[test]
	fn verify_cli() {
		Opt::command().debug_assert()
	}
}
