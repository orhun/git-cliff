use crate::error::Result;
use regex::Regex;

/// Configuration values.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	pub changelog: ChangelogConfig,
	/// Configuration values about git.
	pub git:       GitConfig,
}

/// Changelog configuration.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ChangelogConfig {
	/// Changelog header.
	pub header: Option<String>,
	/// Changelog body, template.
	pub body:   String,
	/// Changelog footer.
	pub footer: Option<String>,
	/// Trim the template.
	pub trim:   Option<bool>,
}

/// Git configuration
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GitConfig {
	/// Whether to enable conventional commits.
	pub conventional_commits: bool,
	/// Git commit parsers.
	pub commit_parsers:       Option<Vec<CommitParser>>,
	/// Whether to filter out commits.
	pub filter_commits:       Option<bool>,
	/// Blob pattern for git tags.
	pub tag_pattern:          Option<String>,
	#[serde(with = "serde_regex", default)]
	/// Regex to skip matched tags.
	pub skip_tags:            Option<Regex>,
}

/// Parser for grouping commits.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
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

impl Config {
	/// Parses the config file and returns the values.
	pub fn parse(file_name: String) -> Result<Config> {
		let mut config = config::Config::default();
		config
			.merge(config::File::with_name(&file_name))?
			.merge(config::Environment::with_prefix("CLIFF").separator("_"))?;
		Ok(config.try_into()?)
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
		let file_name = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.parent()
			.unwrap()
			.to_path_buf()
			.join("config")
			.join(crate::DEFAULT_CONFIG)
			.to_str()
			.unwrap()
			.to_string();
		env::set_var("CLIFF_CHANGELOG_FOOTER", "test");
		let config = Config::parse(file_name)?;
		assert_eq!("test", config.changelog.footer.unwrap());
		Ok(())
	}
}
