use std::path::Path;
use std::str;

use rust_embed::RustEmbed;

use crate::config::Config;
use crate::error::{Error, Result};

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
        Self::get_config()?.parse()
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
            None => Err(Error::EmbeddedError(format!("config {name} not found"))),
        }?;
        Ok(contents)
    }

    /// Parses the extracted content into [`Config`] along with the name.
    ///
    /// [`Config`]: Config
    pub fn parse(name: String) -> Result<(Config, String)> {
        let parsed = Self::get_config(name.clone())?.parse()?;
        Ok((parsed, name))
    }
}
