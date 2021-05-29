use crate::error::Result;
use regex::Regex;

/// Configuration values.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
	pub changelog: ChangelogConfig,
}

/// Changelog configuration.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ChangelogConfig {
	pub header:          String,
	pub body:            String,
	pub footer:          String,
	pub commit_parsers:  Vec<CommitParser>,
	pub filter_group:    bool,
	pub git_tag_pattern: String,
	#[serde(with = "serde_regex")]
	pub skip_tags_regex: Regex,
}

/// Parser for grouping commits.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitParser {
	#[serde(with = "serde_regex")]
	pub regex: Regex,
	pub group: Option<String>,
	pub skip:  Option<bool>,
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
