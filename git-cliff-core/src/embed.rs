use crate::config::Config;
use crate::error::{
	Error,
	Result,
};
use rust_embed::RustEmbed;
use std::path::Path;
use std::str;

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
		Config::parse_from_str(&Self::get_config()?)
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
	pub fn get_config(mut name: String) -> Result<String> {
		if !Path::new(&name)
			.extension()
			.is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
		{
			name = format!("{name}.toml");
		}
		let contents = match Self::get(&name) {
			Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
			None => Err(Error::EmbeddedError(format!("config {} not found", name,))),
		}?;
		Ok(contents)
	}

	/// Parses the extracted content into [`Config`] along with the name.
	///
	/// [`Config`]: Config
	pub fn parse(name: String) -> Result<(Config, String)> {
		let raw_config = Self::get_config(name.to_string())?;
		let parsed = Config::parse_from_str(&raw_config)?;
		Ok((parsed, name))
	}
}
