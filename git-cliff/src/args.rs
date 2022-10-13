use clap::{
	AppSettings,
	ArgEnum,
	Parser,
};
use git_cliff_core::glob::Pattern;
use git_cliff_core::DEFAULT_CONFIG;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ArgEnum)]
pub enum Strip {
	Header,
	Footer,
	All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ArgEnum)]
pub enum Sort {
	Oldest,
	Newest,
}

/// Command-line arguments to parse.
#[derive(Debug, Parser)]
#[clap(
    version,
    author,
    about,
    global_setting = AppSettings::DeriveDisplayOrder,
    rename_all_env = "screaming-snake",
    next_help_heading = Some("OPTIONS"),
    override_usage = "git-cliff [FLAGS] [OPTIONS] [--] [RANGE]",
    mut_arg("help", |arg| arg.help("Prints help information").help_heading("FLAGS")),
    mut_arg("version", |arg| arg.help("Prints version information").help_heading("FLAGS"))
)]
pub struct Opt {
	/// Increases the logging verbosity.
	#[clap(short, long, parse(from_occurrences), alias = "debug", help_heading = Some("FLAGS"))]
	pub verbose:               u8,
	/// Sets the configuration file.
	#[clap(short, long, env = "GIT_CLIFF_CONFIG", value_name = "PATH", default_value = DEFAULT_CONFIG)]
	pub config:                PathBuf,
	/// Sets the working directory.
	#[clap(short, long, env = "GIT_CLIFF_WORKDIR", value_name = "PATH")]
	pub workdir:               Option<PathBuf>,
	/// Sets the git repository.
	#[clap(short, long, env = "GIT_CLIFF_REPOSITORY", value_name = "PATH")]
	pub repository:            Option<PathBuf>,
	/// Sets the path to include related commits.
	#[clap(
		long,
		env = "GIT_CLIFF_INCLUDE_PATH",
		value_name = "PATTERN",
		multiple_values = true
	)]
	pub include_path:          Option<Vec<Pattern>>,
	/// Sets the path to exclude related commits.
	#[clap(
		long,
		env = "GIT_CLIFF_EXCLUDE_PATH",
		value_name = "PATTERN",
		multiple_values = true
	)]
	pub exclude_path:          Option<Vec<Pattern>>,
	/// Sets custom commit messages to include in the changelog.
	#[clap(
		long,
		env = "GIT_CLIFF_WITH_COMMIT",
		value_name = "MSG",
		multiple_values = true
	)]
	pub with_commit:           Option<Vec<String>>,
	/// Prepends entries to the given changelog file.
	#[clap(short, long, env = "GIT_CLIFF_PREPEND", value_name = "PATH")]
	pub prepend:               Option<PathBuf>,
	/// Writes output to the given file.
	#[clap(short, long, env = "GIT_CLIFF_OUTPUT", value_name = "PATH")]
	pub output:                Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[clap(
		short,
		long,
		env = "GIT_CLIFF_TAG",
		value_name = "TAG",
		allow_hyphen_values = true
	)]
	pub tag:                   Option<String>,
	/// Sets the template for the changelog body.
	#[clap(
		short,
		long,
		env = "GIT_CLIFF_TEMPLATE",
		value_name = "TEMPLATE",
		allow_hyphen_values = true
	)]
	pub body:                  Option<String>,
	/// Writes the default configuration file to cliff.toml
	#[clap(short, long, help_heading = Some("FLAGS"))]
	pub init:                  bool,
	/// Processes the commits starting from the latest tag.
	#[clap(short, long, help_heading = Some("FLAGS"))]
	pub latest:                bool,
	/// Processes the commits that belong to the current tag.
	#[clap(long, help_heading = Some("FLAGS"))]
	pub current:               bool,
	/// Processes the commits that do not belong to a tag.
	#[clap(short, long, help_heading = Some("FLAGS"))]
	pub unreleased:            bool,
	/// Sorts the tags chronologically.
	#[clap(long, help_heading = Some("FLAGS"))]
	pub date_order:            bool,
	/// Prints changelog context as JSON.
	#[clap(long, help_heading = Some("FLAGS"))]
	pub context:               bool,
	/// Strips the given parts from the changelog.
	#[clap(short, long, value_name = "PART", arg_enum)]
	pub strip:                 Option<Strip>,
	/// Sets sorting of the commits inside sections.
	#[clap(
		long,
		arg_enum,
		default_value_t = Sort::Oldest
	)]
	pub sort:                  Sort,
	/// Sets the commit range to process.
	#[clap(value_name = "RANGE", help_heading = Some("ARGS"))]
	pub range:                 Option<String>,
	/// Adds [CommitPreprocessor] steps for github issue & pull request
	/// references (#i)
	#[clap(long, env = "GIT_CLIFF_GITHUB_REPOSITORY_URL")]
	pub github_repository_url: Option<String>,
}
