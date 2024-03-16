use crate::command;
use crate::error::Result;
use clap::ValueEnum;
use regex::{
	Regex,
	RegexBuilder,
};
use secrecy::SecretString;
use serde::{
	Deserialize,
	Serialize,
};
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Manifest file information and regex for matching contents.
#[derive(Debug)]
struct ManifestInfo {
	/// Path of the manifest.
	path:  PathBuf,
	/// Regular expression for matching metadata in the manifest.
	regex: Regex,
}

lazy_static::lazy_static! {
	/// Array containing manifest information for Rust and Python projects.
	static ref MANIFEST_INFO: Vec<ManifestInfo> = vec![
		ManifestInfo {
			path: PathBuf::from("Cargo.toml"),
			regex: RegexBuilder::new(
				r"^\[(?:workspace|package)\.metadata\.git\-cliff\.",
			)
			.multi_line(true)
			.build()
			.expect("failed to build regex"),
		},
		ManifestInfo {
			path: PathBuf::from("pyproject.toml"),
			regex: RegexBuilder::new(r"^\[(?:tool)\.git\-cliff\.")
				.multi_line(true)
				.build()
				.expect("failed to build regex"),
		},
	];

}

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
	/// Whether to filter out commits.
	pub filter_commits:          Option<bool>,
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
	/// Reads the config file contents from project manifest (e.g. Cargo.toml,
	/// pyproject.toml)
	pub fn read_from_manifest() -> Result<Option<String>> {
		for info in (*MANIFEST_INFO).iter() {
			if info.path.exists() {
				let contents = fs::read_to_string(&info.path)?;
				if info.regex.is_match(&contents) {
					return Ok(Some(
						info.regex.replace_all(&contents, "[").to_string(),
					));
				}
			}
		}
		Ok(None)
	}

	/// Parses the config file from string and returns the values.
	pub fn parse_from_str(contents: &str) -> Result<Config> {
		Ok(config::Config::builder()
			.add_source(config::File::from_str(contents, config::FileFormat::Toml))
			.add_source(
				config::Environment::with_prefix("GIT_CLIFF").separator("__"),
			)
			.build()?
			.try_deserialize()?)
	}

	/// Parses the config file and returns the values.
	pub fn parse(path: &Path) -> Result<Config> {
		if MANIFEST_INFO
			.iter()
			.any(|v| path.file_name() == v.path.file_name())
		{
			if let Some(contents) = Self::read_from_manifest()? {
				return Self::parse_from_str(&contents);
			}
		}

		Ok(config::Config::builder()
			.add_source(config::File::from(path))
			.add_source(
				config::Environment::with_prefix("GIT_CLIFF").separator("__"),
			)
			.build()?
			.try_deserialize()?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::env;
	#[test]
	fn parse_config() -> Result<()> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.parent()
			.expect("parent directory not found")
			.to_path_buf()
			.join("config")
			.join(crate::DEFAULT_CONFIG);

		const FOOTER_VALUE: &str = "test";
		const RELEASE_TAGS_PATTERN_VALUE: &str = ".*[0-9].*";
		const RELEASE_SKIP_TAGS_PATTERN_VALUE: &str =
			"v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+";

		env::set_var("GIT_CLIFF__CHANGELOG__FOOTER", FOOTER_VALUE);
		env::set_var(
			"GIT_CLIFF__RELEASE__TAGS_PATTERN",
			RELEASE_TAGS_PATTERN_VALUE,
		);
		env::set_var(
			"GIT_CLIFF__RELEASE__SKIP_TAGS_PATTERN",
			RELEASE_SKIP_TAGS_PATTERN_VALUE,
		);

		let config = Config::parse(&path)?;

		assert_eq!(Some(String::from(FOOTER_VALUE)), config.changelog.footer);
		assert_eq!(
			Some(String::from(RELEASE_TAGS_PATTERN_VALUE)),
			config
				.release
				.tags_pattern
				.map(|tags_pattern| tags_pattern.to_string())
		);
		assert_eq!(
			Some(String::from(RELEASE_SKIP_TAGS_PATTERN_VALUE)),
			config
				.release
				.skip_tags_pattern
				.map(|skip_tags_pattern| skip_tags_pattern.to_string())
		);
		Ok(())
	}

	#[test]
	fn remote_config() {
		let remote1 = Remote::new("abc", "xyz1");
		let remote2 = Remote::new("abc", "xyz2");
		assert!(!remote1.eq(&remote2));
		assert_eq!("abc/xyz1", remote1.to_string());
		assert!(remote1.is_set());
		assert!(!Remote::new("", "test").is_set());
		assert!(!Remote::new("test", "").is_set());
		assert!(!Remote::new("", "").is_set());
	}
}
