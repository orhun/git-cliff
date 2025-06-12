use crate::command;
use crate::embed::EmbeddedConfig;
use crate::error::Result;
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

/// Default initial tag.
const DEFAULT_INITIAL_TAG: &str = "0.1.0";

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

/// Configuration values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	#[serde(default)]
	pub changelog: ChangelogConfig,
	/// Configuration values about git.
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
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChangelogConfig {
	/// Changelog header.
	pub header:         Option<String>,
	/// Changelog body, template.
	pub body:           String,
	/// Changelog footer.
	pub footer:         Option<String>,
	/// Trim the template.
	pub trim:           bool,
	/// Always render the body template.
	pub render_always:  bool,
	/// Changelog postprocessors.
	pub postprocessors: Vec<TextProcessor>,
	/// Output file path.
	pub output:         Option<PathBuf>,
}

/// Git configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitConfig {
	/// Parse commits according to the conventional commits specification.
	pub conventional_commits:  bool,
	/// Require all commits to be conventional.
	/// Takes precedence over filter_unconventional.
	pub require_conventional:  bool,
	/// Exclude commits that do not match the conventional commits specification
	/// from the changelog.
	pub filter_unconventional: bool,
	/// Split commits on newlines, treating each line as an individual commit.
	pub split_commits:         bool,

	/// An array of regex based parsers to modify commit messages prior to
	/// further processing.
	pub commit_preprocessors:     Vec<TextProcessor>,
	/// An array of regex based parsers for extracting data from the commit
	/// message.
	pub commit_parsers:           Vec<CommitParser>,
	/// Prevent commits having the `BREAKING CHANGE:` footer from being excluded
	/// by commit parsers.
	pub protect_breaking_commits: bool,
	/// An array of regex based parsers to extract links from the commit message
	/// and add them to the commit's context.
	pub link_parsers:             Vec<LinkParser>,
	/// Exclude commits that are not matched by any commit parser.
	pub filter_commits:           bool,
	/// Regex to select git tags that represent releases.
	#[serde(with = "serde_regex", default)]
	pub tag_pattern:              Option<Regex>,
	/// Regex to select git tags that do not represent proper releases.
	#[serde(with = "serde_regex", default)]
	pub skip_tags:                Option<Regex>,
	/// Regex to exclude git tags after applying the tag_pattern.
	#[serde(with = "serde_regex", default)]
	pub ignore_tags:              Option<Regex>,
	/// Regex to count matched tags.
	#[serde(with = "serde_regex", default)]
	pub count_tags:               Option<Regex>,
	/// Include only the tags that belong to the current branch.
	pub use_branch_tags:          bool,
	/// Order releases topologically instead of chronologically.
	pub topo_order:               bool,
	/// Order commits chronologically instead of topologically.
	pub topo_order_commits:       bool,
	/// How to order commits in each group/release within the changelog.
	pub sort_commits:             String,
	/// Limit the total number of commits included in the changelog.
	pub limit_commits:            Option<usize>,
	/// Read submodule commits.
	pub recurse_submodules:       Option<bool>,
}

/// Remote configuration.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
	/// GitHub remote.
	#[serde(default)]
	pub github:    Remote,
	/// GitLab remote.
	#[serde(default)]
	pub gitlab:    Remote,
	/// Gitea remote.
	#[serde(default)]
	pub gitea:     Remote,
	/// Bitbucket remote.
	#[serde(default)]
	pub bitbucket: Remote,
}

impl RemoteConfig {
	/// Returns `true` if any remote is set.
	pub fn is_any_set(&self) -> bool {
		#[cfg(feature = "github")]
		if self.github.is_set() {
			return true;
		}
		#[cfg(feature = "gitlab")]
		if self.gitlab.is_set() {
			return true;
		}
		#[cfg(feature = "gitea")]
		if self.gitea.is_set() {
			return true;
		}
		#[cfg(feature = "bitbucket")]
		if self.bitbucket.is_set() {
			return true;
		}
		false
	}

	/// Enables the native TLS for all remotes.
	pub fn enable_native_tls(&mut self) {
		#[cfg(feature = "github")]
		{
			self.github.native_tls = Some(true);
		}
		#[cfg(feature = "gitlab")]
		{
			self.gitlab.native_tls = Some(true);
		}
		#[cfg(feature = "gitea")]
		{
			self.gitea.native_tls = Some(true);
		}
		#[cfg(feature = "bitbucket")]
		{
			self.bitbucket.native_tls = Some(true);
		}
	}
}

/// A single remote.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Remote {
	/// Owner of the remote.
	pub owner:      String,
	/// Repository name.
	pub repo:       String,
	/// Access token.
	#[serde(skip_serializing)]
	pub token:      Option<SecretString>,
	/// Whether if the remote is set manually.
	#[serde(skip_deserializing, default = "default_true")]
	pub is_custom:  bool,
	/// Remote API URL.
	pub api_url:    Option<String>,
	/// Whether to use native TLS.
	pub native_tls: Option<bool>,
}

/// Returns `true` for serde's `default` attribute.
fn default_true() -> bool {
	true
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
			owner:      owner.into(),
			repo:       repo.into(),
			token:      None,
			is_custom:  false,
			api_url:    None,
			native_tls: None,
		}
	}

	/// Returns `true` if the remote has an owner and repo.
	pub fn is_set(&self) -> bool {
		!self.owner.is_empty() && !self.repo.is_empty()
	}
}

/// Version bump type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BumpType {
	/// Bump major version.
	Major,
	/// Bump minor version.
	Minor,
	/// Bump patch version.
	Patch,
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

	/// Configures the initial version of the project.
	///
	/// When set, the version will be set to this value if no tags are found.
	pub initial_tag: Option<String>,

	/// Configure a custom regex pattern for major version increments.
	///
	/// This will check only the type of the commit against the given pattern.
	///
	/// ### Note
	///
	/// `commit type` according to the spec is only `[a-zA-Z]+`
	pub custom_major_increment_regex: Option<String>,

	/// Configure a custom regex pattern for minor version increments.
	///
	/// This will check only the type of the commit against the given pattern.
	///
	/// ### Note
	///
	/// `commit type` according to the spec is only `[a-zA-Z]+`
	pub custom_minor_increment_regex: Option<String>,

	/// Force to always bump in major, minor or patch.
	pub bump_type: Option<BumpType>,
}

impl Bump {
	/// Returns the initial tag.
	///
	/// This function also logs the returned value.
	pub fn get_initial_tag(&self) -> String {
		if let Some(tag) = self.initial_tag.clone() {
			warn!(
				"No releases found, using initial tag '{tag}' as the next version."
			);
			tag
		} else {
			warn!(
				"No releases found, using {DEFAULT_INITIAL_TAG} as the next \
				 version."
			);
			DEFAULT_INITIAL_TAG.into()
		}
	}
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
	/// Regex for matching the commit footer.
	#[serde(with = "serde_regex", default)]
	pub footer:        Option<Regex>,
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

/// `TextProcessor`, e.g. for modifying commit messages.
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
		for info in &(*MANIFEST_INFO) {
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
		// Adding sources one after another overwrites the previous values.
		// Thus adding the default config initializes the config with default values.
		let default_config_str = EmbeddedConfig::get_config()?;

		Ok(config::Config::builder()
			.add_source(config::File::from_str(
				&default_config_str,
				config::FileFormat::Toml,
			))
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

		// Adding sources one after another overwrites the previous values.
		// Thus adding the default config initializes the config with default values.
		let default_config_str = EmbeddedConfig::get_config()?;
		Ok(config::Config::builder()
			.add_source(config::File::from_str(
				&default_config_str,
				config::FileFormat::Toml,
			))
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
		const TAG_PATTERN_VALUE: &str = ".*[0-9].*";
		const IGNORE_TAGS_VALUE: &str = "v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+";

		unsafe {
			env::set_var("GIT_CLIFF__CHANGELOG__FOOTER", FOOTER_VALUE);
			env::set_var("GIT_CLIFF__GIT__TAG_PATTERN", TAG_PATTERN_VALUE);
			env::set_var("GIT_CLIFF__GIT__IGNORE_TAGS", IGNORE_TAGS_VALUE);
		};

		let config = Config::parse(&path)?;

		assert_eq!(Some(String::from(FOOTER_VALUE)), config.changelog.footer);
		assert_eq!(
			Some(String::from(TAG_PATTERN_VALUE)),
			config
				.git
				.tag_pattern
				.map(|tag_pattern| tag_pattern.to_string())
		);
		assert_eq!(
			Some(String::from(IGNORE_TAGS_VALUE)),
			config
				.git
				.ignore_tags
				.map(|ignore_tags| ignore_tags.to_string())
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
