use clap::{
	builder::{
		TypedValueParser,
		ValueParserFactory,
	},
	error::{
		ContextKind,
		ContextValue,
		ErrorKind,
	},
	ArgAction,
	Parser,
	ValueEnum,
};
use git_cliff_core::{
	config::Remote,
	DEFAULT_CONFIG,
	DEFAULT_OUTPUT,
};
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
	pub help:           Option<bool>,
	#[arg(
		short = 'V',
		long,
		action = ArgAction::Version,
		global = true,
		help = "Prints version information",
		help_heading = "FLAGS"
	)]
	pub version:        Option<bool>,
	/// Increases the logging verbosity.
	#[arg(short, long, action = ArgAction::Count, alias = "debug", help_heading = Some("FLAGS"))]
	pub verbose:        u8,
	/// Writes the default configuration file to cliff.toml
	#[arg(
	    short,
	    long,
	    value_name = "CONFIG",
	    num_args = 0..=1,
	    required = false
	)]
	pub init:           Option<Option<String>>,
	/// Sets the configuration file.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_CONFIG",
	    value_name = "PATH",
	    default_value = DEFAULT_CONFIG,
	    value_parser = Opt::parse_dir
	)]
	pub config:         PathBuf,
	/// Sets the working directory.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_WORKDIR",
	    value_name = "PATH",
	    value_parser = Opt::parse_dir
	)]
	pub workdir:        Option<PathBuf>,
	/// Sets the git repository.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_REPOSITORY",
		value_name = "PATH",
		num_args(1..),
		value_parser = Opt::parse_dir
	)]
	pub repository:     Option<Vec<PathBuf>>,
	/// Sets the path to include related commits.
	#[arg(
		long,
		env = "GIT_CLIFF_INCLUDE_PATH",
		value_name = "PATTERN",
		num_args(1..)
	)]
	pub include_path:   Option<Vec<Pattern>>,
	/// Sets the path to exclude related commits.
	#[arg(
		long,
		env = "GIT_CLIFF_EXCLUDE_PATH",
		value_name = "PATTERN",
		num_args(1..)
	)]
	pub exclude_path:   Option<Vec<Pattern>>,
	/// Sets custom commit messages to include in the changelog.
	#[arg(
		long,
		env = "GIT_CLIFF_WITH_COMMIT",
		value_name = "MSG",
		num_args(1..)
	)]
	pub with_commit:    Option<Vec<String>>,
	/// Sets commits that will be skipped in the changelog.
	#[arg(
		long,
		env = "GIT_CLIFF_SKIP_COMMIT",
		value_name = "SHA1",
		num_args(1..)
	)]
	pub skip_commit:    Option<Vec<String>>,
	/// Prepends entries to the given changelog file.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_PREPEND",
	    value_name = "PATH",
	    value_parser = Opt::parse_dir
	)]
	pub prepend:        Option<PathBuf>,
	/// Writes output to the given file.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_OUTPUT",
	    value_name = "PATH",
	    value_parser = Opt::parse_dir,
	    num_args = 0..=1,
	    default_missing_value = DEFAULT_OUTPUT
	)]
	pub output:         Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_TAG",
		value_name = "TAG",
		allow_hyphen_values = true
	)]
	pub tag:            Option<String>,
	/// Bumps the version for unreleased changes.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub bump:           bool,
	/// Prints bumped version for unreleased changes.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub bumped_version: bool,
	/// Sets the template for the changelog body.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_TEMPLATE",
		value_name = "TEMPLATE",
		allow_hyphen_values = true
	)]
	pub body:           Option<String>,
	/// Processes the commits starting from the latest tag.
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub latest:         bool,
	/// Processes the commits that belong to the current tag.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub current:        bool,
	/// Processes the commits that do not belong to a tag.
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub unreleased:     bool,
	/// Sorts the tags topologically.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub topo_order:     bool,
	/// Disables the external command execution.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub no_exec:        bool,
	/// Prints changelog context as JSON.
	#[arg(short = 'x', long, help_heading = Some("FLAGS"))]
	pub context:        bool,
	/// Strips the given parts from the changelog.
	#[arg(short, long, value_name = "PART", value_enum)]
	pub strip:          Option<Strip>,
	/// Sets sorting of the commits inside sections.
	#[arg(
		long,
		value_enum,
		default_value_t = Sort::Oldest
	)]
	pub sort:           Sort,
	/// Sets the commit range to process.
	#[arg(value_name = "RANGE", help_heading = Some("ARGS"))]
	pub range:          Option<String>,
	/// Sets the GitHub API token.
	#[arg(
		long,
		env = "GITHUB_TOKEN",
		value_name = "TOKEN",
		hide_env_values = true
	)]
	pub github_token:   Option<String>,
	/// Sets the GitHub repository.
	#[arg(
	    long,
	    env = "GITHUB_REPO",
	    value_parser = clap::value_parser!(RemoteValue),
	    value_name = "OWNER/REPO"
	)]
	pub github_repo:    Option<RemoteValue>,
}

/// Custom type for the remote value.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoteValue(pub Remote);

impl ValueParserFactory for RemoteValue {
	type Parser = RemoteValueParser;
	fn value_parser() -> Self::Parser {
		RemoteValueParser
	}
}

/// Parser for the remote value.
#[derive(Clone, Debug)]
pub struct RemoteValueParser;

impl TypedValueParser for RemoteValueParser {
	type Value = RemoteValue;
	fn parse_ref(
		&self,
		cmd: &clap::Command,
		arg: Option<&clap::Arg>,
		value: &std::ffi::OsStr,
	) -> Result<Self::Value, clap::Error> {
		let inner = clap::builder::StringValueParser::new();
		let value = inner.parse_ref(cmd, arg, value)?;
		let parts = value.split('/').rev().collect::<Vec<&str>>();
		if let (Some(owner), Some(repo)) = (parts.get(1), parts.first()) {
			Ok(RemoteValue(Remote::new(*owner, *repo)))
		} else {
			let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
			if let Some(arg) = arg {
				err.insert(
					ContextKind::InvalidArg,
					ContextValue::String(arg.to_string()),
				);
			}
			err.insert(ContextKind::InvalidValue, ContextValue::String(value));
			Err(err)
		}
	}
}

impl Opt {
	/// Custom string parser for directories.
	///
	/// Expands the tilde (`~`) character in the beginning of the
	/// input string into contents of the path returned by [`home_dir`].
	///
	/// [`home_dir`]: dirs::home_dir
	fn parse_dir(dir: &str) -> Result<PathBuf, String> {
		Ok(PathBuf::from(shellexpand::tilde(dir).to_string()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;
	use std::ffi::OsStr;

	#[test]
	fn verify_cli() {
		Opt::command().debug_assert()
	}

	#[test]
	fn path_tilde_expansion() {
		let home_dir = dirs::home_dir().expect("cannot retrieve home directory");
		let dir = Opt::parse_dir("~/").expect("cannot expand tilde");
		assert_eq!(home_dir, dir);
	}

	#[test]
	fn remote_value_parser() -> Result<(), clap::Error> {
		let remote_value_parser = RemoteValueParser;
		assert_eq!(
			RemoteValue(Remote::new("test", "repo")),
			remote_value_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("test/repo")
			)?
		);
		assert!(remote_value_parser
			.parse_ref(&Opt::command(), None, OsStr::new("test"))
			.is_err());
		assert_eq!(
			RemoteValue(Remote::new("test", "testrepo")),
			remote_value_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("https://github.com/test/testrepo")
			)?
		);
		assert!(remote_value_parser
			.parse_ref(&Opt::command(), None, OsStr::new(""))
			.is_err());
		Ok(())
	}
}
