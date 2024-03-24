use super::DEFAULT_CONFIG_FILENAME;
use crate::error::{
	Error,
	Result,
};
use regex::{
	Regex,
	RegexBuilder,
};
use rust_embed::RustEmbed;
use std::fs;
use std::{
	path::PathBuf,
	str,
};

/// Manifest file information and regex for matching contents.
#[derive(Debug)]
struct ManifestInfo {
	/// Path of the manifest.
	path:  PathBuf,
	/// Regular expression for matching metadata in the manifest.
	regex: Regex,
}

lazy_static::lazy_static! {
	/// Array containing manifest information for Rust and Python projects.
	static ref MANIFEST_INFO: Vec<ManifestInfo> = vec![
		ManifestInfo {
			path: PathBuf::from("Cargo.toml"),
			regex: RegexBuilder::new(
				r"^\[(?:workspace|package)\.metadata\.git\-cliff\.",
			)
			.multi_line(true)
			.build()
			.expect("failed to build regex"),
		},
		ManifestInfo {
			path: PathBuf::from("pyproject.toml"),
			regex: RegexBuilder::new(r"^\[(?:tool)\.git\-cliff\.")
				.multi_line(true)
				.build()
				.expect("failed to build regex"),
		},
	];

}

/// Reads the config file contents from project manifest (e.g. Cargo.toml,
/// pyproject.toml)
pub fn read_from_manifest() -> Result<Option<String>> {
	for info in (*MANIFEST_INFO).iter() {
		if info.path.exists() {
			let contents = fs::read_to_string(&info.path)?;
			if info.regex.is_match(&contents) {
				return Ok(Some(info.regex.replace_all(&contents, "[").to_string()));
			}
		}
	}
	Ok(None)
}

/// Default configuration file embedder/extractor.
///
/// Embeds `config/`[`DEFAULT_CONFIG`] into the binary.
///
/// [`DEFAULT_CONFIG`]: crate::DEFAULT_CONFIG
#[derive(Debug, RustEmbed)]
#[folder = "../config/"]
pub struct EmbeddedConfig;

impl EmbeddedConfig {
	/// Extracts the embedded content.
	pub fn get_config_str() -> Result<String> {
		match Self::get(DEFAULT_CONFIG_FILENAME) {
			Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
			None => Err(Error::EmbeddedError(String::from(
				"Embedded config not found",
			))),
		}
	}
}

/// Built-in configuration file embedder/extractor.
///
/// Embeds the files under `/examples/` into the binary.
#[derive(RustEmbed)]
#[folder = "../examples/"]
pub struct BuiltinConfig;

impl BuiltinConfig {
	/// Extracts the embedded content.
	pub fn get_config_str(mut name: String) -> Result<(String, String)> {
		if !name.ends_with(".toml") {
			name = format!("{name}.toml");
		}
		let contents = match Self::get(&name) {
			Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
			None => Err(Error::EmbeddedError(format!("config {} not found", name,))),
		}?;
		Ok((contents, name))
	}
}
