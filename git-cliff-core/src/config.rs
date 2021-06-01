use crate::error::Result;
use regex::Regex;

/// Configuration values.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	pub changelog: ChangelogConfig,
}

/// Changelog configuration.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ChangelogConfig {
	/// Changelog header.
	pub header:          String,
	/// Changelog body, template.
	pub body:            String,
	/// Changelog footer.
	pub footer:          String,
	/// Git commit parsers.
	pub commit_parsers:  Vec<CommitParser>,
	/// Whether to filter out commits.
	pub filter_group:    bool,
	/// Blob pattern for git tags.
	pub git_tag_pattern: String,
	#[serde(with = "serde_regex")]
	/// Regex to skip matched tags.
	pub skip_tags_regex: Regex,
}

/// Parser for grouping commits.
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitParser {
	/// Regex for matching commit message.
	#[serde(with = "serde_regex")]
	pub regex: Regex,
	/// Group of the commit.
	pub group: Option<String>,
	/// Whether to skip this commit group.
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
