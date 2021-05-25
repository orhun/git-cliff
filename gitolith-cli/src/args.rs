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
		default_value = "gitolith.toml"
	)]
	pub config:     String,
	#[structopt(short, long, env, value_name = "TAG", allow_hyphen_values = true)]
	/// Sets the tag for the latest version.
	pub tag:        Option<String>,
}
