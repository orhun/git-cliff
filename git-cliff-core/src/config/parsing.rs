use crate::error::Result;
use regex::RegexBuilder;
use serde::Deserialize;
use std::{
	ffi::OsStr,
	fs,
	path::Path,
};

/// Regex for matching the metadata in Cargo.toml
const CARGO_METADATA_REGEX: &str =
	r"^\[(?:workspace|package)\.metadata\.git\-cliff\.";

/// Regex for matching the metadata in pyproject.toml
const PYPROJECT_METADATA_REGEX: &str = r"^\[(?:tool)\.git\-cliff\.";

/// Loads configuration from a file and GIT_CLIFF_* environment variables.
pub fn parse<'de, T: Deserialize<'de>>(path: &Path) -> Result<T> {
	let file_name = path.file_name();
	let config_builder = if file_name == Some(OsStr::new("Cargo.toml")) ||
		file_name == Some(OsStr::new("pyproject.toml"))
	{
		let contents = fs::read_to_string(path)?;
		let metadata_regex = RegexBuilder::new(
			if path.file_name() == Some(OsStr::new("Cargo.toml")) {
				CARGO_METADATA_REGEX
			} else {
				PYPROJECT_METADATA_REGEX
			},
		)
		.multi_line(true)
		.build()?;
		let contents = metadata_regex.replace_all(&contents, "[");
		config::Config::builder()
			.add_source(config::File::from_str(&contents, config::FileFormat::Toml))
	} else {
		config::Config::builder().add_source(config::File::from(path))
	};

	Ok(config_builder
		.add_source(config::Environment::with_prefix("GIT_CLIFF").separator("__"))
		.build()?
		.try_deserialize()?)
}
