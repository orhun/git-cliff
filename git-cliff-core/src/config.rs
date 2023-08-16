use crate::command;
use crate::error::Result;
use regex::{
	Regex,
	RegexBuilder,
};
use serde::{
	Deserialize,
	Serialize,
};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Regex for matching the metadata in Cargo.toml
const CARGO_METADATA_REGEX: &str =
	r"^\[(?:workspace|package)\.metadata\.git\-cliff\.";

/// Regex for matching the metadata in pyproject.toml
const PYPROJECT_METADATA_REGEX: &str = r"^\[(?:tool)\.git\-cliff\.";

/// Configuration values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	/// Configuration values about changelog generation.
	#[serde(default)]
	pub changelog: ChangelogConfig,
	/// Configuration values about git.
	#[serde(default)]
	pub git:       GitConfig,
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

/// Git configuration
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
	pub tag_pattern:              Option<String>,
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

/// Parser for grouping commits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitParser {
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
	/// Parses the config file and returns the values.
	pub fn parse(path: &Path) -> Result<Config> {
		let config_builder = if path.file_name() == Some(OsStr::new("Cargo.toml")) ||
			path.file_name() == Some(OsStr::new("pyproject.toml"))
		{
			let contents = fs::read_to_string(path)?;
			let metadata_regex = RegexBuilder::new(
				if path.file_name() == Some(OsStr::new("Cargo.toml")) {
					CARGO_METADATA_REGEX
				} else {
					PYPROJECT_METADATA_REGEX
				},
			)
			.multi_line(true)
			.build()?;
			let contents = metadata_regex.replace_all(&contents, "[");
			config::Config::builder().add_source(config::File::from_str(
				&contents,
				config::FileFormat::Toml,
			))
		} else {
			config::Config::builder().add_source(config::File::from(path))
		};
		Ok(config_builder
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
	use std::path::PathBuf;
	#[test]
	fn parse_config() -> Result<()> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.parent()
			.expect("parent directory not found")
			.to_path_buf()
			.join("config")
			.join(crate::DEFAULT_CONFIG);

		const FOOTER_VALUE: &str = "test";
		const TAG_PATTERN_VALUE: &str = "*[0-9]*";
		const IGNORE_TAGS_VALUE: &str = "v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+";

		env::set_var("GIT_CLIFF__CHANGELOG__FOOTER", FOOTER_VALUE);
		env::set_var("GIT_CLIFF__GIT__TAG_PATTERN", TAG_PATTERN_VALUE);
		env::set_var("GIT_CLIFF__GIT__IGNORE_TAGS", IGNORE_TAGS_VALUE);

		let config = Config::parse(&path)?;

		assert_eq!(Some(String::from(FOOTER_VALUE)), config.changelog.footer);
		assert_eq!(
			Some(String::from(TAG_PATTERN_VALUE)),
			config.git.tag_pattern
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
}
