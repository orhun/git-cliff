use super::models_v2::{
	Bump,
	CommitParser,
	LinkParser,
	RemoteConfig,
	TextProcessor,
};
use regex::Regex;
use serde::{
	Deserialize,
	Serialize,
};

/// Configuration values.
#[deprecated(since = "3.0.0", note = "deprecated in favor of models_v2")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	#[allow(deprecated)]
	#[serde(default)]
	pub changelog: ChangelogConfig,
	/// Configuration values about git.
	#[allow(deprecated)]
	#[serde(default)]
	pub git:       GitConfig,
	/// Configuration values about remote.
	#[serde(default)]
	pub remote:    RemoteConfig,
	/// Configuration values about bump version.
	#[serde(default)]
	pub bump:      Bump,
}

/// Changelog configuration.
#[allow(deprecated)]
#[deprecated(since = "3.0.0", note = "deprecated in favor of models_v2")]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChangelogConfig {
	/// Changelog header.
	pub header:         Option<String>,
	/// Changelog body, template.
	pub body:           Option<String>,
	/// Changelog footer.
	pub footer:         Option<String>,
	/// Trim the template.
	pub trim:           Option<bool>,
	/// Changelog postprocessors.
	pub postprocessors: Option<Vec<TextProcessor>>,
}

/// Git configuration
#[deprecated(since = "3.0.0", note = "deprecated in favor of models_v2")]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitConfig {
	/// Whether to enable parsing conventional commits.
	pub conventional_commits:  Option<bool>,
	/// Whether to filter out unconventional commits.
	pub filter_unconventional: Option<bool>,
	/// Whether to split commits by line, processing each line as an individual
	/// commit.
	pub split_commits:         Option<bool>,

	/// Git commit preprocessors.
	pub commit_preprocessors:     Option<Vec<TextProcessor>>,
	/// Git commit parsers.
	pub commit_parsers:           Option<Vec<CommitParser>>,
	/// Whether to protect all breaking changes from being skipped by a commit
	/// parser.
	pub protect_breaking_commits: Option<bool>,
	/// Link parsers.
	pub link_parsers:             Option<Vec<LinkParser>>,
	/// Whether to filter out commits.
	pub filter_commits:           Option<bool>,
	/// Blob pattern for git tags.
	#[serde(with = "serde_regex", default)]
	pub tag_pattern:              Option<Regex>,
	/// Regex to skip matched tags.
	#[serde(with = "serde_regex", default)]
	pub skip_tags:                Option<Regex>,
	/// Regex to ignore matched tags.
	#[serde(with = "serde_regex", default)]
	pub ignore_tags:              Option<Regex>,
	/// Whether to sort tags topologically.
	pub topo_order:               Option<bool>,
	/// Sorting of the commits inside sections.
	pub sort_commits:             Option<String>,
	/// Limit the number of commits included in the changelog.
	pub limit_commits:            Option<usize>,
}
