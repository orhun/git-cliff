use crate::error::Result;
use regex::Regex;
use std::path::Path;

/// Configuration values.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	#[serde(default)]
	pub changelog: ChangelogConfig,
	/// Configuration values about git.
	#[serde(default)]
	pub git:       GitConfig,
}

/// Changelog configuration.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChangelogConfig {
	/// Changelog header.
	pub header: Option<String>,
	/// Changelog body, template.
	pub body:   Option<String>,
	/// Changelog footer.
	pub footer: Option<String>,
	/// Trim the template.
	pub trim:   Option<bool>,
}

/// Git configuration
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitConfig {
	/// Whether to enable parsing conventional commits.
	pub conventional_commits:  Option<bool>,
	/// Whether to filter out unconventional commits.
	pub filter_unconventional: Option<bool>,
	/// Git commit parsers.
	pub commit_parsers:        Option<Vec<CommitParser>>,
	/// Link parsers.
	pub link_parsers:          Option<Vec<LinkParser>>,
	/// Whether to filter out commits.
	pub filter_commits:        Option<bool>,
	/// Blob pattern for git tags.
	pub tag_pattern:           Option<String>,
	#[serde(with = "serde_regex", default)]
	/// Regex to skip matched tags.
	pub skip_tags:             Option<Regex>,
	#[serde(with = "serde_regex", default)]
	/// Regex to ignore matched tags.
	pub ignore_tags:           Option<Regex>,
	/// Whether to sort tags chronologically.
	pub date_order:            Option<bool>,
	/// Sorting of the commits inside sections.
	pub sort_commits:          Option<String>,
}

/// Parser for grouping commits.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommitParser {
	/// Regex for matching the commit message.
	#[serde(with = "serde_regex", default)]
	pub message:       Option<Regex>,
	/// Regex for matching the commit body.
	#[serde(with = "serde_regex", default)]
	pub body:          Option<Regex>,
	/// Group of the commit.
	pub group:         Option<String>,
	/// Scope of the commit.
	pub default_scope: Option<String>,
	/// Whether to skip this commit group.
	pub skip:          Option<bool>,
}

/// Parser for extracting links in commits.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinkParser {
	/// Regex for finding links in the commit message.
	#[serde(with = "serde_regex")]
	pub pattern: Regex,
	/// The string used to generate the link URL.
	pub href:    String,
	/// The string used to generate the link text.
	pub text:    Option<String>,
}

impl Config {
	/// Parses the config file and returns the values.
	pub fn parse(path: &Path) -> Result<Config> {
		Ok(config::Config::builder()
			.add_source(config::File::from(path))
			.add_source(config::Environment::with_prefix("CLIFF").separator("_"))
			.build()?
			.try_deserialize()?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::env;
	use std::path::PathBuf;
	#[test]
	fn parse_config() -> Result<()> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.parent()
			.unwrap()
			.to_path_buf()
			.join("config")
			.join(crate::DEFAULT_CONFIG);
		env::set_var("CLIFF_CHANGELOG_FOOTER", "test");
		let config = Config::parse(&path)?;
		assert_eq!("test", config.changelog.footer.unwrap());
		Ok(())
	}
}
