use crate::command;
use crate::error::Result;
use clap::ValueEnum;
use regex::Regex;
use secrecy::SecretString;
use serde::{
	Deserialize,
	Serialize,
};
use std::fmt;

/// Options for ordering git tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TagsOrderBy {
	/// Whether to sort git tags according to their creation date.
	Time,
	/// Whether to sort git tags according to the git topology.
	Topology,
}

/// Options for ordering commits chronologically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommitSortOrder {
	/// Whether to sort starting with the oldest element.
	Oldest,
	/// Whether to sort starting with the newest element.
	Newest,
}

/// Configuration values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	#[serde(default)]
	pub changelog: ChangelogConfig,
	/// Configuration values about releases.
	#[serde(default)]
	pub release:   ReleaseConfig,
	/// Configuration values about git commits.
	#[serde(default)]
	pub commit:    CommitConfig,
	/// Configuration values about remote.
	#[serde(default)]
	pub remote:    RemoteConfig,
	/// Configuration values about bump version.
	#[serde(default)]
	pub bump:      Bump,
}

/// Changelog configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChangelogConfig {
	/// A static header for the changelog.
	pub header:                    Option<String>,
	/// A Tera template to be rendered for each release in the changelog.
	pub body_template:             Option<String>,
	/// A Tera template to be rendered as the changelog's footer.
	pub footer_template:           Option<String>,
	/// Whether to remove leading and trailing whitespaces from all lines of the
	/// changelog's body.
	pub trim_body_whitespace:      Option<bool>,
	/// A list of postprocessors using regex to modify the changelog.
	pub postprocessors:            Option<Vec<TextProcessor>>,
	/// Whether to exclude changes that do not belong to any group from the
	/// changelog.
	pub exclude_ungrouped_changes: Option<bool>,
}

/// Release configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReleaseConfig {
	/// Regex to select git tags that represent releases.
	/// Example: "v[0-9].*"
	#[serde(with = "serde_regex", default)]
	pub tags_pattern:      Option<Regex>,
	/// Regex to select git tags that do not represent proper releases. Takes
	/// precedence over `release.tags_pattern`. Changes belonging to these
	/// releases will be included in the next non-skipped release. Example: "rc"
	#[serde(with = "serde_regex", default)]
	pub skip_tags_pattern: Option<Regex>,
	/// Whether to order releases chronologically or topologically.
	pub order_by:          Option<TagsOrderBy>,
}

/// Git commit configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitConfig {
	/// Whether to parse commits according to the conventional commits
	/// specification.
	pub parse_conventional_commits:     Option<bool>,
	/// Whether to exclude commits that do not match the conventional commits
	/// specification from the changelog.
	pub exclude_unconventional_commits: Option<bool>,
	/// Whether to split commits on newlines, treating each line as an
	/// individual commit.
	pub split_by_newline:               Option<bool>,

	/// A list of preprocessors to modify commit messages using regex prior to
	/// further processing.
	pub message_preprocessors:   Option<Vec<TextProcessor>>,
	/// A list of parsers using regex for extracting data from the commit
	/// message.
	pub commit_parsers:          Option<Vec<CommitParser>>,
	/// Whether to prevent breaking changes from being excluded by commit
	/// parsers.
	pub retain_breaking_changes: Option<bool>,
	/// A list of parsers using regex for extracting external references found
	/// in commit messages, and turning them into links. The gemerated links can
	/// be used in the body template as `commit.links`.
	pub link_parsers:            Option<Vec<LinkParser>>,
	/// Regex to select git tags that should be excluded from the changelog.
	#[serde(with = "serde_regex", default)]
	pub exclude_tags_pattern:    Option<Regex>,
	/// Whether to order commits newest to oldest or oldest to newest in their
	/// group.
	pub sort_order:              Option<CommitSortOrder>,
	/// Whether to limit the total number of commits to be included in the
	/// changelog.
	pub max_commit_count:        Option<usize>,
}

/// Remote configuration.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
	/// GitHub remote.
	pub github: Remote,
}

/// A single remote.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Remote {
	/// Owner of the remote.
	pub owner: String,
	/// Repository name.
	pub repo:  String,
	/// Access token.
	#[serde(skip_serializing)]
	pub token: Option<SecretString>,
}

impl fmt::Display for Remote {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}/{}", self.owner, self.repo)
	}
}

impl PartialEq for Remote {
	fn eq(&self, other: &Self) -> bool {
		self.to_string() == other.to_string()
	}
}

impl Remote {
	/// Constructs a new instance.
	pub fn new<S: Into<String>>(owner: S, repo: S) -> Self {
		Self {
			owner: owner.into(),
			repo:  repo.into(),
			token: None,
		}
	}

	/// Returns `true` if the remote has an owner and repo.
	pub fn is_set(&self) -> bool {
		!self.owner.is_empty() && !self.repo.is_empty()
	}
}

/// Bump version configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Bump {
	/// Configures automatic minor version increments for feature changes.
	///
	/// When `true`, a feature will always trigger a minor version update.
	/// When `false`, a feature will trigger:
	///
	/// - A patch version update if the major version is 0.
	/// - A minor version update otherwise.
	pub features_always_bump_minor: Option<bool>,

	/// Configures 0 -> 1 major version increments for breaking changes.
	///
	/// When `true`, a breaking change commit will always trigger a major
	/// version update (including the transition from version 0 to 1)
	/// When `false`, a breaking change commit will trigger:
	///
	/// - A minor version update if the major version is 0.
	/// - A major version update otherwise.
	pub breaking_always_bump_major: Option<bool>,
}

/// Parser for grouping commits.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitParser {
	/// SHA1 of the commit.
	pub sha:           Option<String>,
	/// Regex for matching the commit message.
	#[serde(with = "serde_regex", default)]
	pub message:       Option<Regex>,
	/// Regex for matching the commit body.
	#[serde(with = "serde_regex", default)]
	pub body:          Option<Regex>,
	/// Group of the commit.
	pub group:         Option<String>,
	/// Default scope of the commit.
	pub default_scope: Option<String>,
	/// Commit scope for overriding the default scope.
	pub scope:         Option<String>,
	/// Whether to skip this commit group.
	pub skip:          Option<bool>,
	/// Field name of the commit to match the regex against.
	pub field:         Option<String>,
	/// Regex for matching the field value.
	#[serde(with = "serde_regex", default)]
	pub pattern:       Option<Regex>,
}

/// TextProcessor, e.g. for modifying commit messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextProcessor {
	/// Regex for matching a text to replace.
	#[serde(with = "serde_regex")]
	pub pattern:         Regex,
	/// Replacement text.
	pub replace:         Option<String>,
	/// Command that will be run for replacing the commit message.
	pub replace_command: Option<String>,
}

impl TextProcessor {
	/// Replaces the text with using the given pattern or the command output.
	pub fn replace(
		&self,
		rendered: &mut String,
		command_envs: Vec<(&str, &str)>,
	) -> Result<()> {
		if let Some(text) = &self.replace {
			*rendered = self.pattern.replace_all(rendered, text).to_string();
		} else if let Some(command) = &self.replace_command {
			if self.pattern.is_match(rendered) {
				*rendered =
					command::run(command, Some(rendered.to_string()), command_envs)?;
			}
		}
		Ok(())
	}
}

/// Parser for extracting links in commits.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
	/// Creates a v2 Config from a deprecated v1 Config.
	#[allow(deprecated)]
	pub fn from(config_v1: super::models_v1::Config) -> Config {
		Config {
			changelog: ChangelogConfig {
				header:                    config_v1.changelog.header,
				body_template:             config_v1.changelog.body,
				footer_template:           config_v1.changelog.footer,
				trim_body_whitespace:      config_v1.changelog.trim,
				postprocessors:            config_v1.changelog.postprocessors,
				exclude_ungrouped_changes: config_v1.git.filter_commits,
			},
			release:   ReleaseConfig {
				tags_pattern:      config_v1.git.tag_pattern,
				skip_tags_pattern: config_v1.git.ignore_tags,
				order_by:          Some(if config_v1.git.topo_order.unwrap() {
					TagsOrderBy::Topology
				} else {
					TagsOrderBy::Time
				}),
			},
			commit:    CommitConfig {
				sort_order:                     config_v1.git.sort_commits.map(
					|s| {
						CommitSortOrder::from_str(&s, true)
							.expect("Incorrect config value for 'sort_commits'")
					},
				),
				max_commit_count:               config_v1.git.limit_commits,
				split_by_newline:               config_v1.git.split_commits,
				exclude_tags_pattern:           config_v1.git.skip_tags,
				message_preprocessors:          config_v1.git.commit_preprocessors,
				link_parsers:                   config_v1.git.link_parsers,
				parse_conventional_commits:     config_v1.git.conventional_commits,
				exclude_unconventional_commits: config_v1.git.filter_unconventional,
				commit_parsers:                 config_v1.git.commit_parsers,
				retain_breaking_changes:        config_v1
					.git
					.protect_breaking_commits,
			},
			remote:    config_v1.remote.clone(),
			bump:      config_v1.bump.clone(),
		}
	}
}
