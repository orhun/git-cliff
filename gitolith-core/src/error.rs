use thiserror::Error as ThisError;

/// Githolit-core related errors that we are exposing to the rest of the
/// workspaces.
#[derive(Debug, ThisError, PartialEq)]
pub enum Error {
	/// When commit's not follow the conventional commit structure we throw this
	/// error.
	#[error("Cannot parse the commit")]
	ParseError,
}

/// Result type of the githolit-core libraries.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod test {
	use super::*;
	fn mock_function() -> super::Result<()> {
		return Err(Error::ParseError);
	}

	#[test]
	fn throw_parse_error() {
		let actual_error = mock_function().unwrap_err();
		let expected_error = Error::ParseError;
		assert_eq!(actual_error, expected_error);
	}
}
