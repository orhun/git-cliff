use crate::error::{
	Error,
	Result,
};
use clap::Args;
use std::{
	fs,
	io::Write,
	path::PathBuf,
};

/// Migrates configuration files from the old to the new schema.
#[derive(Args, Debug)]
pub struct MigrateArgs {
	/// The file to read the original configuration from.
	#[arg(long = "in")]
	pub in_path: PathBuf,

	/// The file to write the migrated configuration to.
	#[arg(long = "out")]
	pub out_path: PathBuf,
}

/// Migrates configuration files from the old to the new schema.
pub fn run(args: &MigrateArgs) -> Result<()> {
	// load the old configuration
	if !args.in_path.exists() {
		return Err(Error::ArgumentError(format!(
			"File {0} does not exist.",
			&args.in_path.to_str().unwrap()
		)));
	}

	let old_config =
		super::parsing::parse::<super::models_v1::Config>(&args.in_path)?;

	// convert to the new config format
	let new_config = super::models_v2::Config::from(old_config);
	let new_toml = toml::to_string(&new_config).unwrap();

	// write the new config file
	let mut new_config_file = fs::OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(&args.out_path)?;

	new_config_file.write_all(new_toml.as_bytes())?;
	new_config_file.flush()?;

	Ok(())
}
