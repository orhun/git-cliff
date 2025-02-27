use thiserror::Error as ThisError;

/// Library related errors that we are exposing to the rest of the workspaces.
#[derive(Debug, ThisError)]
pub enum Error {
	/// Error that may occur while I/O operations.
	#[error("IO error: `{0}`")]
	IoError(#[from] std::io::Error),
	/// Error that may occur when attempting to interpret a sequence of u8 as a
	/// string.
	#[error("UTF-8 error: `{0}`")]
	Utf8Error(#[from] std::str::Utf8Error),
	/// Error variant that represents errors coming out of libgit2.
	#[cfg(feature = "repo")]
	#[error("Git error: `{0}`")]
	GitError(#[from] git2::Error),
	/// Error that may occur when failed to set a commit range.
	#[cfg(feature = "repo")]
	#[error(
		"Failed to set the commit range: {1}
{0:?} is not a valid commit range. Did you provide the correct arguments?"
	)]
	SetCommitRangeError(String, #[source] git2::Error),
	/// Error variant that represents other repository related errors.
	#[cfg(feature = "repo")]
	#[error("Git repository error: `{0}`")]
	RepoError(String),
	/// Error that may occur while parsing the config file.
	#[error("Cannot parse config: `{0}`")]
	ConfigError(#[from] config::ConfigError),
	/// A possible error while initializing the logger.
	#[error("Logger error: `{0}`")]
	LoggerError(String),
	/// When commit's not follow the conventional commit structure we throw this
	/// error.
	#[error("Cannot parse the commit: `{0}`")]
	ParseError(#[from] git_conventional::Error),
	/// Error that may occur while grouping commits.
	#[error("Grouping error: `{0}`")]
	GroupError(String),
	/// Error that may occur while generating changelog.
	#[error("Changelog error: `{0}`")]
	ChangelogError(String),
	/// Error that may occur while parsing the template.
	#[error("Template parse error:\n{0}")]
	TemplateParseError(String),
	/// Error that may occur while rendering the template.
	#[error("Template render error:\n{0}")]
	TemplateRenderError(String),
	/// Error that may occur while rendering the template.
	#[error("Template render error:\n{0}\n{1}")]
	TemplateRenderDetailedError(String, String),
	/// Error that may occur during more general template operations.
	#[error("Template error: `{0}`")]
	TemplateError(#[from] tera::Error),
	/// Error that may occur while parsing the command line arguments.
	#[error("Argument error: `{0}`")]
	ArgumentError(String),
	/// Error that may occur while extracting the embedded content.
	#[error("Embedded error: `{0}`")]
	EmbeddedError(String),
	/// Errors that may occur when deserializing types from TOML format.
	#[error("Cannot parse TOML: `{0}`")]
	DeserializeError(#[from] toml::de::Error),
	/// Errors that may occur while de/serializing JSON format.
	#[error("Cannot de/serialize JSON: `{0}`")]
	JsonError(#[from] serde_json::Error),
	/// Errors that may occur during parsing or compiling a regular expression.
	#[error("Cannot parse/compile regex: `{0}`")]
	RegexError(#[from] regex::Error),
	/// Error that may occur due to system time related anomalies.
	#[error("System time error: `{0}`")]
	SystemTimeError(#[from] std::time::SystemTimeError),
	/// Error that may occur while parsing integers.
	#[error("Failed to parse integer: `{0}`")]
	IntParseError(#[from] std::num::TryFromIntError),
	/// Error that may occur while processing parsers that define field and
	/// value matchers.
	#[error("Field error: `{0}`")]
	FieldError(String),
	/// Error that may occur while parsing a `SemVer` version or version
	/// requirement.
	#[error("Semver error: `{0}`")]
	SemverError(#[from] semver::Error),
	/// The errors that may occur when processing a HTTP request.
	#[error("HTTP client error: `{0}`")]
	#[cfg(feature = "remote")]
	HttpClientError(#[from] reqwest::Error),
	/// The errors that may occur while constructing the HTTP client with
	/// middleware.
	#[error("HTTP client with middleware error: `{0}`")]
	#[cfg(feature = "remote")]
	HttpClientMiddlewareError(#[from] reqwest_middleware::Error),
	/// A possible error when converting a `HeaderValue` from a string or byte
	/// slice.
	#[error("HTTP header error: `{0}`")]
	#[cfg(feature = "remote")]
	HttpHeaderError(#[from] reqwest::header::InvalidHeaderValue),
	/// Error that may occur during handling pages.
	#[error("Pagination error: `{0}`")]
	PaginationError(String),
	/// The errors that may occur while parsing URLs.
	#[error("URL parse error: `{0}`")]
	UrlParseError(#[from] url::ParseError),
	/// Error that may occur when a remote is not set.
	#[error("Repository remote is not set.")]
	RemoteNotSetError,
	/// Error that may occur while handling location of directories.
	#[error("Directory error: `{0}`")]
	DirsError(String),
	/// Error that may occur while constructing patterns.
	#[error("Pattern error: `{0}`")]
	PatternError(#[from] glob::PatternError),
	/// Error that may occur when unconventional commits are found.
	/// See `require_conventional` option for more information.
	#[error(
		"Requiring all commits be conventional but found {0} unconventional \
		 commits."
	)]
	UnconventionalCommitsError(i32),
}

/// Result type of the core library.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod test {
	use super::*;
	use git_conventional::{
		Commit,
		ErrorKind,
	};
	fn mock_function() -> super::Result<Commit<'static>> {
		Ok(Commit::parse("test")?)
	}

	#[test]
	fn throw_parse_error() {
		let actual_error = mock_function().expect_err("expected error");
		let expected_error_kind = ErrorKind::MissingType;
		match actual_error {
			Error::ParseError(e) => {
				assert_eq!(expected_error_kind, e.kind());
			}
			_ => {
				unreachable!()
			}
		}
	}
}
