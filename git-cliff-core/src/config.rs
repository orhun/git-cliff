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
	pub tag_pattern:          String,
	#[serde(with = "serde_regex", default)]
	/// Regex to skip matched tags.
	pub skip_tags:            Option<Regex>,
}

/// Parser for grouping commits.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitParser {
	/// Regex for matching the commit message.
	#[serde(with = "serde_regex", default)]
	pub message: Option<Regex>,
	/// Regex for matching the commit body.
	#[serde(with = "serde_regex", default)]
	pub body:    Option<Regex>,
	/// Group of the commit.
	pub group:   Option<String>,
	/// Whether to skip this commit group.
	pub skip:    Option<bool>,
}

impl Config {
	/// Parses the config file and returns the values.
	pub fn parse(file_name: String) -> Result<Config> {
		let mut config = config::Config::default();
		config
			.merge(config::File::with_name(&file_name))?
			.merge(config::Environment::with_prefix(env!("CARGO_PKG_NAME")))?;
		Ok(config.try_into()?)
	}
}
