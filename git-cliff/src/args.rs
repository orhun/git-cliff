use clap::{
	ArgAction,
	Parser,
	ValueEnum,
	builder::{
		Styles,
		TypedValueParser,
		ValueParserFactory,
		styling::{
			Ansi256Color,
			AnsiColor,
		},
	},
	error::{
		ContextKind,
		ContextValue,
		ErrorKind,
	},
};
use git_cliff_core::{
	DEFAULT_CONFIG,
	DEFAULT_OUTPUT,
	config::BumpType,
	config::Remote,
};
use glob::Pattern;
use regex::Regex;
use secrecy::SecretString;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Strip {
	Header,
	Footer,
	All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Sort {
	Oldest,
	Newest,
}

const STYLES: Styles = Styles::styled()
	.header(Ansi256Color(208).on_default().bold())
	.usage(Ansi256Color(208).on_default().bold())
	.literal(AnsiColor::White.on_default())
	.placeholder(AnsiColor::Green.on_default());

/// Command-line arguments to parse.
#[derive(Debug, Parser)]
#[command(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
	help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
    override_usage = "git-cliff [FLAGS] [OPTIONS] [--] [RANGE]",
    next_help_heading = Some("OPTIONS"),
	disable_help_flag = true,
	disable_version_flag = true,
    styles(STYLES),
)]
pub struct Opt {
	#[arg(
		short,
		long,
		action = ArgAction::Help,
		global = true,
		help = "Prints help information",
		help_heading = "FLAGS"
	)]
	pub help:             Option<bool>,
	#[arg(
		short = 'V',
		long,
		action = ArgAction::Version,
		global = true,
		help = "Prints version information",
		help_heading = "FLAGS"
	)]
	pub version:          Option<bool>,
	/// Increases the logging verbosity.
	#[arg(short, long, action = ArgAction::Count, alias = "debug", help_heading = Some("FLAGS"))]
	pub verbose:          u8,
	/// Writes the default configuration file to cliff.toml
	#[arg(
	    short,
	    long,
	    value_name = "CONFIG",
	    num_args = 0..=1,
	    required = false
	)]
	pub init:             Option<Option<String>>,
	/// Sets the configuration file.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_CONFIG",
	    value_name = "PATH",
	    default_value = DEFAULT_CONFIG,
	    value_parser = Opt::parse_dir
	)]
	pub config:           PathBuf,
	/// Sets the URL for the configuration file.
	#[arg(long, env = "GIT_CLIFF_CONFIG_URL", value_name = "URL", hide = !cfg!(feature = "remote"))]
	pub config_url:       Option<Url>,
	/// Sets the working directory.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_WORKDIR",
	    value_name = "PATH",
	    value_parser = Opt::parse_dir
	)]
	pub workdir:          Option<PathBuf>,
	/// Sets the git repository.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_REPOSITORY",
		value_name = "PATH",
		num_args(1..),
		value_parser = Opt::parse_dir
	)]
	pub repository:       Option<Vec<PathBuf>>,
	/// Sets the path to include related commits.
	#[arg(
		long,
		env = "GIT_CLIFF_INCLUDE_PATH",
		value_name = "PATTERN",
		num_args(1..)
	)]
	pub include_path:     Option<Vec<Pattern>>,
	/// Sets the path to exclude related commits.
	#[arg(
		long,
		env = "GIT_CLIFF_EXCLUDE_PATH",
		value_name = "PATTERN",
		num_args(1..)
	)]
	pub exclude_path:     Option<Vec<Pattern>>,
	/// Sets the regex for matching git tags.
	#[arg(long, env = "GIT_CLIFF_TAG_PATTERN", value_name = "PATTERN")]
	pub tag_pattern:      Option<Regex>,
	/// Sets custom commit messages to include in the changelog.
	#[arg(
		long,
		env = "GIT_CLIFF_WITH_COMMIT",
		value_name = "MSG",
		num_args(1..)
	)]
	pub with_commit:      Option<Vec<String>>,
	/// Sets custom message for the latest release.
	#[arg(
		long,
		env = "GIT_CLIFF_WITH_TAG_MESSAGE",
		value_name = "MSG",
		num_args = 0..=1,
	)]
	pub with_tag_message: Option<String>,
	/// Sets the tags to ignore in the changelog.
	#[arg(long, env = "GIT_CLIFF_IGNORE_TAGS", value_name = "PATTERN")]
	pub ignore_tags:      Option<Regex>,
	/// Sets the tags to count in the changelog.
	#[arg(long, env = "GIT_CLIFF_COUNT_TAGS", value_name = "PATTERN")]
	pub count_tags:       Option<Regex>,
	/// Sets commits that will be skipped in the changelog.
	#[arg(
		long,
		env = "GIT_CLIFF_SKIP_COMMIT",
		value_name = "SHA1",
		num_args(1..)
	)]
	pub skip_commit:      Option<Vec<String>>,
	/// Prepends entries to the given changelog file.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_PREPEND",
	    value_name = "PATH",
	    value_parser = Opt::parse_dir
	)]
	pub prepend:          Option<PathBuf>,
	/// Writes output to the given file.
	#[arg(
	    short,
	    long,
	    env = "GIT_CLIFF_OUTPUT",
	    value_name = "PATH",
	    value_parser = Opt::parse_dir,
	    num_args = 0..=1,
	    default_missing_value = DEFAULT_OUTPUT
	)]
	pub output:           Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_TAG",
		value_name = "TAG",
		allow_hyphen_values = true
	)]
	pub tag:              Option<String>,
	/// Bumps the version for unreleased changes. Optionally with specified
	/// version.
	#[arg(
        long,
        value_name = "BUMP",
        value_enum,
        num_args = 0..=1,
        default_missing_value = "auto",
        value_parser = clap::value_parser!(BumpOption))]
	pub bump:             Option<BumpOption>,
	/// Prints bumped version for unreleased changes.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub bumped_version:   bool,
	/// Sets the template for the changelog body.
	#[arg(
		short,
		long,
		env = "GIT_CLIFF_TEMPLATE",
		value_name = "TEMPLATE",
		allow_hyphen_values = true
	)]
	pub body:             Option<String>,
	/// Processes the commits starting from the latest tag.
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub latest:           bool,
	/// Processes the commits that belong to the current tag.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub current:          bool,
	/// Processes the commits that do not belong to a tag.
	#[arg(short, long, help_heading = Some("FLAGS"))]
	pub unreleased:       bool,
	/// Sorts the tags topologically.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub topo_order:       bool,
	/// Include only the tags that belong to the current branch.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub use_branch_tags:  bool,
	/// Disables the external command execution.
	#[arg(long, help_heading = Some("FLAGS"))]
	pub no_exec:          bool,
	/// Prints changelog context as JSON.
	#[arg(short = 'x', long, help_heading = Some("FLAGS"))]
	pub context:          bool,
	/// Generates changelog from a JSON context.
	#[arg(
        long,
	    value_name = "PATH",
	    value_parser = Opt::parse_dir,
		env = "GIT_CLIFF_CONTEXT",
    )]
	pub from_context:     Option<PathBuf>,
	/// Strips the given parts from the changelog.
	#[arg(short, long, value_name = "PART", value_enum)]
	pub strip:            Option<Strip>,
	/// Sets sorting of the commits inside sections.
	#[arg(
		long,
		value_enum,
		default_value_t = Sort::Oldest
	)]
	pub sort:             Sort,
	/// Sets the commit range to process.
	#[arg(value_name = "RANGE", help_heading = Some("ARGS"))]
	pub range:            Option<String>,
	/// Sets the GitHub API token.
	#[arg(
		long,
		env = "GITHUB_TOKEN",
		value_name = "TOKEN",
		hide_env_values = true,
		hide = !cfg!(feature = "github"),
	)]
	pub github_token:     Option<SecretString>,
	/// Sets the GitHub repository.
	#[arg(
		long,
		env = "GITHUB_REPO",
		value_parser = clap::value_parser!(RemoteValue),
		value_name = "OWNER/REPO",
		hide = !cfg!(feature = "github"),
	)]
	pub github_repo:      Option<RemoteValue>,
	/// Sets the GitLab API token.
	#[arg(
		long,
		env = "GITLAB_TOKEN",
		value_name = "TOKEN",
		hide_env_values = true,
		hide = !cfg!(feature = "gitlab"),
	)]
	pub gitlab_token:     Option<SecretString>,
	/// Sets the GitLab repository.
	#[arg(
		long,
		env = "GITLAB_REPO",
		value_parser = clap::value_parser!(RemoteValue),
		value_name = "OWNER/REPO",
		hide = !cfg!(feature = "gitlab"),
	)]
	pub gitlab_repo:      Option<RemoteValue>,
	/// Sets the Gitea API token.
	#[arg(
		long,
		env = "GITEA_TOKEN",
		value_name = "TOKEN",
		hide_env_values = true,
		hide = !cfg!(feature = "gitea"),
	)]
	pub gitea_token:      Option<SecretString>,
	/// Sets the Gitea repository.
	#[arg(
		long,
		env = "GITEA_REPO",
		value_parser = clap::value_parser!(RemoteValue),
		value_name = "OWNER/REPO",
		hide = !cfg!(feature = "gitea"),
	)]
	pub gitea_repo:       Option<RemoteValue>,
	/// Sets the Bitbucket API token.
	#[arg(
		long,
		env = "BITBUCKET_TOKEN",
		value_name = "TOKEN",
		hide_env_values = true,
		hide = !cfg!(feature = "bitbucket"),
	)]
	pub bitbucket_token:  Option<SecretString>,
	/// Sets the Bitbucket repository.
	#[arg(
		long,
		env = "BITBUCKET_REPO",
		value_parser = clap::value_parser!(RemoteValue),
		value_name = "OWNER/REPO",
		hide = !cfg!(feature = "bitbucket"),
	)]
	pub bitbucket_repo:   Option<RemoteValue>,
	/// Load TLS certificates from the native certificate store.
	#[arg(long, help_heading = Some("FLAGS"), hide = !cfg!(feature = "remote"))]
	pub use_native_tls:   bool,
}

/// Custom type for the remote value.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoteValue(pub Remote);

impl ValueParserFactory for RemoteValue {
	type Parser = RemoteValueParser;
	fn value_parser() -> Self::Parser {
		RemoteValueParser
	}
}

/// Parser for the remote value.
#[derive(Clone, Debug)]
pub struct RemoteValueParser;

impl TypedValueParser for RemoteValueParser {
	type Value = RemoteValue;
	fn parse_ref(
		&self,
		cmd: &clap::Command,
		arg: Option<&clap::Arg>,
		value: &std::ffi::OsStr,
	) -> Result<Self::Value, clap::Error> {
		let inner = clap::builder::StringValueParser::new();
		let mut value = inner.parse_ref(cmd, arg, value)?;
		if let Ok(url) = Url::parse(&value) {
			value = url.path().trim_start_matches('/').to_string();
		}
		let parts = value.rsplit_once('/');
		if let Some((owner, repo)) = parts {
			Ok(RemoteValue(Remote::new(
				owner.to_string(),
				repo.to_string(),
			)))
		} else {
			let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
			if let Some(arg) = arg {
				err.insert(
					ContextKind::InvalidArg,
					ContextValue::String(arg.to_string()),
				);
			}
			err.insert(ContextKind::InvalidValue, ContextValue::String(value));
			Err(err)
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BumpOption {
	Auto,
	Specific(BumpType),
}

impl ValueParserFactory for BumpOption {
	type Parser = BumpOptionParser;
	fn value_parser() -> Self::Parser {
		BumpOptionParser
	}
}

/// Parser for bump type.
#[derive(Clone, Debug)]
pub struct BumpOptionParser;

impl TypedValueParser for BumpOptionParser {
	type Value = BumpOption;
	fn parse_ref(
		&self,
		cmd: &clap::Command,
		arg: Option<&clap::Arg>,
		value: &std::ffi::OsStr,
	) -> Result<Self::Value, clap::Error> {
		let inner = clap::builder::StringValueParser::new();
		let value = inner.parse_ref(cmd, arg, value)?;
		match value.as_str() {
			"auto" => Ok(BumpOption::Auto),
			"major" => Ok(BumpOption::Specific(BumpType::Major)),
			"minor" => Ok(BumpOption::Specific(BumpType::Minor)),
			"patch" => Ok(BumpOption::Specific(BumpType::Patch)),
			_ => {
				let mut err =
					clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
				if let Some(arg) = arg {
					err.insert(
						ContextKind::InvalidArg,
						ContextValue::String(arg.to_string()),
					);
				}
				err.insert(ContextKind::InvalidValue, ContextValue::String(value));
				Err(err)
			}
		}
	}
}

impl Opt {
	/// Custom string parser for directories.
	///
	/// Expands the tilde (`~`) character in the beginning of the
	/// input string into contents of the path returned by [`home_dir`].
	///
	/// [`home_dir`]: dirs::home_dir
	fn parse_dir(dir: &str) -> Result<PathBuf, String> {
		Ok(PathBuf::from(shellexpand::tilde(dir).to_string()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;
	use std::ffi::OsStr;

	#[test]
	fn verify_cli() {
		Opt::command().debug_assert();
	}

	#[test]
	fn path_tilde_expansion() {
		let home_dir = dirs::home_dir().expect("cannot retrieve home directory");
		let dir = Opt::parse_dir("~/").expect("cannot expand tilde");
		assert_eq!(home_dir, dir);
	}

	#[test]
	fn remote_value_parser() -> Result<(), clap::Error> {
		let remote_value_parser = RemoteValueParser;
		assert_eq!(
			RemoteValue(Remote::new("test", "repo")),
			remote_value_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("test/repo")
			)?
		);
		assert_eq!(
			RemoteValue(Remote::new("gitlab/group/test", "repo")),
			remote_value_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("gitlab/group/test/repo")
			)?
		);
		assert_eq!(
			RemoteValue(Remote::new("test", "testrepo")),
			remote_value_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("https://github.com/test/testrepo")
			)?
		);
		assert_eq!(
			RemoteValue(Remote::new("archlinux/packaging/packages", "arch-repro-status")),
			remote_value_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("https://gitlab.archlinux.org/archlinux/packaging/packages/arch-repro-status")
			)?
		);
		assert!(
			remote_value_parser
				.parse_ref(&Opt::command(), None, OsStr::new("test"))
				.is_err()
		);
		assert!(
			remote_value_parser
				.parse_ref(&Opt::command(), None, OsStr::new(""))
				.is_err()
		);
		Ok(())
	}

	#[test]
	fn bump_option_parser() -> Result<(), clap::Error> {
		let bump_option_parser = BumpOptionParser;
		assert_eq!(
			BumpOption::Auto,
			bump_option_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("auto")
			)?
		);
		assert!(
			bump_option_parser
				.parse_ref(&Opt::command(), None, OsStr::new("test"))
				.is_err()
		);
		assert_eq!(
			BumpOption::Specific(BumpType::Major),
			bump_option_parser.parse_ref(
				&Opt::command(),
				None,
				OsStr::new("major")
			)?
		);
		Ok(())
	}
}
