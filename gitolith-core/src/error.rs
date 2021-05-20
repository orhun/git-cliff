use thiserror::Error as ThisError;

/// Library related errors that we are exposing to the rest of the
/// workspaces.
#[derive(Debug, ThisError)]
pub enum Error {
	/// Error that may occur while I/O operations.
	#[error("IO error: `{0}`")]
	IoError(#[from] std::io::Error),
	/// Error variant that represents errors coming out of libgit2.
	#[error("Git error: `{0}`")]
	GitError(#[from] git2::Error),
	/// Error that may occur while parsing the config file.
	#[error("Cannot parse config: `{0}`")]
	ConfigError(#[from] config::ConfigError),
	/// When commit's not follow the conventional commit structure we throw this
	/// error.
	#[error("Cannot parse the commit: `{0}`")]
	ParseError(#[from] git_conventional::Error),
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
		let actual_error = mock_function().unwrap_err();
		let expected_error_kind = ErrorKind::InvalidFormat;
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
