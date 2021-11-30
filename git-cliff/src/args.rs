use git_cliff_core::glob::Pattern;
use git_cliff_core::DEFAULT_CONFIG;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Command-line arguments to parse.
#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_settings(&[
        AppSettings::ColorAuto,
        AppSettings::ColoredHelp,
        AppSettings::DeriveDisplayOrder,
    ]),
    rename_all_env = "screaming-snake"
)]
pub struct Opt {
	/// Increases the logging verbosity.
	#[structopt(short, long, parse(from_occurrences), alias = "debug")]
	pub verbose:     u8,
	/// Sets the configuration file.
	#[structopt(
		short,
		long,
		env,
		value_name = "PATH",
		default_value = DEFAULT_CONFIG,
	)]
	pub config:      PathBuf,
	/// Sets the working directory.
	#[structopt(short, long, env, value_name = "PATH")]
	pub workdir:     Option<PathBuf>,
	/// Sets the git repository.
	#[structopt(short, long, env, value_name = "PATH")]
	pub repository:  Option<PathBuf>,
	/// Sets the directory to parse commits from.
	#[structopt(long, env, value_name = "PATH")]
	pub commit_path: Option<Pattern>,
	/// Prepends entries to the given changelog file.
	#[structopt(short, long, env, value_name = "PATH")]
	pub prepend:     Option<PathBuf>,
	/// Writes output to the given file.
	#[structopt(short, long, env, value_name = "PATH")]
	pub output:      Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[structopt(short, long, env, value_name = "TAG", allow_hyphen_values = true)]
	pub tag:         Option<String>,
	/// Sets the template for the changelog body.
	#[structopt(
		short,
		long,
		env = "TEMPLATE",
		value_name = "TEMPLATE",
		allow_hyphen_values = true
	)]
	pub body:        Option<String>,
	/// Writes the default configuration file to cliff.toml
	#[structopt(short, long)]
	pub init:        bool,
	/// Processes the commits starting from the latest tag.
	#[structopt(short, long)]
	pub latest:      bool,
	/// Processes the commits that do not belong to a tag.
	#[structopt(short, long)]
	pub unreleased:  bool,
	/// Sorts the tags topologically.
	#[structopt(long)]
	pub topo_order:  bool,
	/// Strips the given parts from the changelog.
	#[structopt(
		short,
		long,
		value_name = "PART",
		possible_values = &["header", "footer", "all"]
	)]
	pub strip:       Option<String>,
	/// Sets the commit range to process.
	#[structopt(value_name = "RANGE")]
	pub range:       Option<String>,
	/// Sets sorting of the commits inside sections.
	#[structopt(
		long,
		possible_values = &["oldest", "newest"],
		default_value = "oldest"
	)]
	pub sort:        String,
}
