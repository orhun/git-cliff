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
	/// Activates the debug mode
	#[structopt(short, long)]
	pub debug:      bool,
	/// Sets the repository to parse commits from.
	#[structopt(short, long, env, value_name = "PATH")]
	pub repository: Option<PathBuf>,
	/// Sets the configuration file.
	#[structopt(
		short,
		long,
		env,
		value_name = "FILE",
		default_value = "cliff.toml"
	)]
	pub config:     String,
	/// Sets the tag for the latest version.
	#[structopt(short, long, env, value_name = "TAG", allow_hyphen_values = true)]
	pub tag:        Option<String>,
	/// Prepend entries to the given changelog from stdin.
	#[structopt(short, long)]
	pub prepend:    bool,
	/// Processes the commits starting from the latest tag.
	#[structopt(short, long)]
	pub latest:     bool,
	/// Processes the commits that do not belong to a tag.
	#[structopt(short, long)]
	pub unreleased: bool,
	/// Strips the header and footer from the changelog.
	#[structopt(short, long)]
	pub strip:      bool,
	/// Sets the commit range to process.
	pub range:      Option<String>,
}
