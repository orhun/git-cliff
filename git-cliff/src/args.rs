use clap::{
	AppSettings,
	ArgEnum,
	Parser,
};
use git_cliff_core::DEFAULT_CONFIG;
use glob::Pattern;
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
	help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
    override_usage = "git-cliff [FLAGS] [OPTIONS] [--] [RANGE]",
    mut_arg("help", |arg| arg.help("Prints help information").help_heading("FLAGS")),
    mut_arg("version", |arg| arg.help("Prints version information").help_heading("FLAGS"))
)]
pub struct Opt {
	/// Increases the logging verbosity.
	#[clap(short, long, parse(from_occurrences), alias = "debug", help_heading = Some("FLAGS"))]
	pub verbose:      u8,
	/// Sets the configuration file.
	#[clap(short, long, env = "GIT_CLIFF_CONFIG", value_name = "PATH", default_value = DEFAULT_CONFIG, value_parser = Opt::parse_dir)]
	pub config:       PathBuf,
	/// Sets the working directory.
	#[clap(short, long, env = "GIT_CLIFF_WORKDIR", value_name = "PATH", value_parser = Opt::parse_dir)]
	pub workdir:      Option<PathBuf>,
	/// Sets the git repository.
	#[clap(
		short,
		long,
		env = "GIT_CLIFF_REPOSITORY",
		value_name = "PATH",
		multiple_values = true,
        value_parser = Opt::parse_dir,
	)]
	pub repository:   Option<Vec<PathBuf>>,
	/// Sets the path to include related commits.
	#[clap(
		long,
		env = "GIT_CLIFF_INCLUDE_PATH",
		value_name = "PATTERN",
		multiple_values = true
	)]
	pub include_path: Option<Vec<Pattern>>,
	/// Sets the path to exclude related commits.
	#[clap(
		long,
		env = "GIT_CLIFF_EXCLUDE_PATH",
		value_name = "PATTERN",
		multiple_values = true
	)]
	pub exclude_path: Option<Vec<Pattern>>,
	/// Sets custom commit messages to include in the changelog.
	#[clap(
		long,
		env = "GIT_CLIFF_WITH_COMMIT",
		value_name = "MSG",
		multiple_values = true
	)]
	pub with_commit:  Option<Vec<String>>,
	/// Prepends entries to the given changelog file.
	#[clap(short, long, env = "GIT_CLIFF_PREPEND", value_name = "PATH", value_parser = Opt::parse_dir)]
	pub prepend:      Option<PathBuf>,
	/// Writes output to the given file.
	#[clap(short, long, env = "GIT_CLIFF_OUTPUT", value_name = "PATH", value_parser = Opt::parse_dir)]
	pub output:       Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[clap(
		short,
		long,
		env = "GIT_CLIFF_TAG",
		value_name = "TAG",
		allow_hyphen_values = true
	)]
	pub tag:          Option<String>,
	/// Sets the template for the changelog body.
	#[clap(
		short,
		long,
		env = "GIT_CLIFF_TEMPLATE",
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
	/// Prints changelog context as JSON.
	#[clap(long, help_heading = Some("FLAGS"))]
	pub context:      bool,
	/// Strips the given parts from the changelog.
	#[clap(short, long, value_name = "PART", arg_enum)]
	pub strip:        Option<Strip>,
	/// Sets sorting of the commits inside sections.
	#[clap(
		long,
		arg_enum,
		default_value_t = Sort::Oldest
	)]
	pub sort:         Sort,
	/// Sets the commit range to process.
	#[clap(value_name = "RANGE", help_heading = Some("ARGS"))]
	pub range:        Option<String>,
}

impl Opt {
	/// Custom string parser for directories.
	///
	/// Expands the tilde (`~`) character in the beginning of the
	/// input string into contents of the path returned by [`home_dir`].
	///
	/// [`home_dir`]: dirs_next::home_dir
	fn parse_dir(dir: &str) -> Result<PathBuf, String> {
		Ok(PathBuf::from(shellexpand::tilde(dir).to_string()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;

	#[test]
	fn verify_cli() {
		Opt::command().debug_assert()
	}

	#[test]
	fn path_tilde_expansion() {
		let home_dir =
			dirs_next::home_dir().expect("cannot retrieve home directory");
		let dir = Opt::parse_dir("~/").expect("cannot expand tilde");
		assert_eq!(home_dir, dir);
	}
}
