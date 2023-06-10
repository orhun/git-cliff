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
	/// Error that may occur while parsing the config file.
	#[error("Cannot parse config: `{0}`")]
	ConfigError(#[from] config::ConfigError),
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
