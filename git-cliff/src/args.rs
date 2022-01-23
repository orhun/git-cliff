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
    rename_all_env = "screaming-snake"
)]
pub struct Opt {
	/// Increases the logging verbosity.
	#[clap(short, long, parse(from_occurrences), alias = "debug", help_heading = Some("FLAGS"))]
	pub verbose:      u8,
	/// Sets the configuration file.
	#[clap(
		short,
		long,
		env,
		value_name = "PATH",
		default_value = DEFAULT_CONFIG,
	)]
	pub config:       PathBuf,
	/// Sets the working directory.
	#[clap(short, long, env, value_name = "PATH")]
	pub workdir:      Option<PathBuf>,
	/// Sets the git repository.
	#[clap(short, long, env, value_name = "PATH")]
	pub repository:   Option<PathBuf>,
	/// Sets the path to include related commits.
	#[clap(long, env, value_name = "PATTERN")]
	pub include_path: Option<Vec<Pattern>>,
	/// Sets the path to exclude related commits.
	#[clap(long, env, value_name = "PATTERN")]
	pub exclude_path: Option<Vec<Pattern>>,
	/// Sets custom commit messages to include in the changelog.
	#[clap(long, env, value_name = "MSG")]
	pub with_commit:  Option<Vec<String>>,
	/// Prepends entries to the given changelog file.
	#[clap(short, long, env, value_name = "PATH")]
	pub prepend:      Option<PathBuf>,
	/// Writes output to the given file.
	#[clap(short, long, env, value_name = "PATH")]
	pub output:       Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[clap(short, long, env, value_name = "TAG", allow_hyphen_values = true)]
	pub tag:          Option<String>,
	/// Sets the template for the changelog body.
	#[clap(
		short,
		long,
		env = "TEMPLATE",
		value_name = "TEMPLATE",
		allow_hyphen_values = true
	)]
	pub body:         Option<String>,
	/// Writes the default configuration file to cliff.toml
	#[clap(short, long, help_heading = Some("FLAGS"))]
	pub init:         bool,
	/// Processes the commits starting from the latest tag.
	#[clap(short, long, help_heading = Some("FLAGS"))]
	pub latest:       bool,
	/// Processes the commits that belong to the current tag.
	#[clap(long, help_heading = Some("FLAGS"))]
	pub current:      bool,
	/// Processes the commits that do not belong to a tag.
	#[clap(short, long, help_heading = Some("FLAGS"))]
	pub unreleased:   bool,
	/// Sorts the tags topologically.
	#[clap(long, help_heading = Some("FLAGS"))]
	pub topo_order:   bool,
	/// Strips the given parts from the changelog.
	#[clap(short, long, value_name = "PART", arg_enum)]
	pub strip:        Option<Strip>,
	/// Sets the commit range to process.
	#[clap(value_name = "RANGE")]
	pub range:        Option<String>,
	/// Sets sorting of the commits inside sections.
	#[clap(
		long,
		arg_enum,
		default_value_t = Sort::Oldest
	)]
	pub sort:         Sort,
}
