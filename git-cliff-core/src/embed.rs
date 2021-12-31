#![allow(missing_docs)] // RustEmbed generated functions does not have doc comments

use crate::config::Config;
use crate::error::{
	Error,
	Result,
};
use rust_embed::RustEmbed;
use std::str;

/// Configuration file embedder/extractor.
///
/// Embeds `config/`[`DEFAULT_CONFIG`] into the binary.
///
/// [`DEFAULT_CONFIG`]: crate::DEFAULT_CONFIG
#[derive(Debug, RustEmbed)]
#[folder = "../config/"]
pub struct EmbeddedConfig;

impl EmbeddedConfig {
	/// Extracts the embedded content.
	pub fn get_config() -> Result<String> {
		match Self::get(crate::DEFAULT_CONFIG) {
			Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
			None => Err(Error::EmbeddedError(String::from(
				"Embedded config not found",
			))),
		}
	}

	/// Parses the extracted content into [`Config`].
	///
	/// [`Config`]: Config
	pub fn parse() -> Result<Config> {
		Ok(toml::from_str(&Self::get_config()?)?)
	}
}
