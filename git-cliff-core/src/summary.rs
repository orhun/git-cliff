use std::fmt::Display;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::error::Error as AppError;

/// Represents the category of errors that may occur while processing a commit.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CommitProcessingErrorKind {
    /// Occurs while spawning or interacting with an OS command via a preprocessor.
    Io,
    /// Occurs when parsing a commit message into a conventional commit fails.
    Parse,
    /// Occurs when serializing or deserializing data to or from JSON fails.
    Json,
    /// Occurs when a referenced commit field is missing or has an unsupported type.
    Field,
    /// Occurs when a commit does not match any grouping rule.
    Group,
    /// Occurs when a commit is skipped intentionally due to configuration.
    Skipped,
    /// Occurs when an error does not fit into any other defined category.
    Other,
}

impl From<AppError> for CommitProcessingErrorKind {
    fn from(err: AppError) -> Self {
        CommitProcessingErrorKind::from(&err)
    }
}

impl From<&AppError> for CommitProcessingErrorKind {
    fn from(err: &AppError) -> Self {
        match err {
            AppError::IoError(_) => CommitProcessingErrorKind::Io,
            AppError::ParseError(_) => CommitProcessingErrorKind::Parse,
            AppError::JsonError(_) => CommitProcessingErrorKind::Json,
            AppError::FieldError(_) => CommitProcessingErrorKind::Field,
            AppError::GroupError(msg) if msg.contains("Skipping commit") => {
                CommitProcessingErrorKind::Skipped
            }
            AppError::GroupError(_) => CommitProcessingErrorKind::Group,
            _ => CommitProcessingErrorKind::Other,
        }
    }
}

impl Display for CommitProcessingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CommitProcessingErrorKind::Io => "I/O error",
            CommitProcessingErrorKind::Parse => "parse error",
            CommitProcessingErrorKind::Json => "JSON error",
            CommitProcessingErrorKind::Field => "field error",
            CommitProcessingErrorKind::Group => "grouping error",
            CommitProcessingErrorKind::Skipped => "intentionally skipped commit",
            CommitProcessingErrorKind::Other => "other error",
        };
        f.write_str(s)
    }
}

impl CommitProcessingErrorKind {
    /// Whether this error kind should be surfaced as a warning summary.
    #[must_use]
    pub fn should_warn(self) -> bool {
        matches!(
            self,
            CommitProcessingErrorKind::Io |
                CommitProcessingErrorKind::Parse |
                CommitProcessingErrorKind::Json |
                CommitProcessingErrorKind::Field |
                CommitProcessingErrorKind::Group |
                CommitProcessingErrorKind::Other
        )
    }
}

/// Aggregated summary of commit processing results for a changelog.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    /// The total number of commits that were processed.
    pub processed: usize,
    /// The number of commits grouped by processing error kind.
    ///
    /// Each entry represents how many commits fell into a particular
    /// [`CommitProcessingErrorKind`] during processing.
    pub by_kind: IndexMap<CommitProcessingErrorKind, usize>,
}

impl Summary {
    /// Records a successfully processed commit.
    pub fn record_ok(&mut self) {
        self.processed += 1;
    }

    /// Records a failed or skipped commit.
    pub fn record_err(&mut self, err: &AppError) {
        self.processed += 1;
        let kind = CommitProcessingErrorKind::from(err);
        *self.by_kind.entry(kind).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod test {
    use std::io::Error as StdIoError;

    use git_conventional::Commit;

    use super::*;
    use crate::error::Error as AppError;

    #[test]
    fn commit_processing_error_kind_from_app_error() {
        let err = AppError::IoError(StdIoError::other("something went wrong".to_string()));
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Io);

        let err = Commit::parse("")
            .map_err(AppError::ParseError)
            .expect_err("expected parse error");
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Parse);

        let err = serde_json::from_str::<serde_json::Value>("{ invalid json }")
            .map_err(AppError::from)
            .expect_err("expected JSON parse error");
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Json);

        let err = AppError::FieldError("missing field".into());
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Field);

        let err = AppError::GroupError("no matching group".into());
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Group);

        let err = AppError::GroupError("Skipping commit due to config".into());
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Skipped);

        let err = AppError::UnmatchedCommitsError(1);
        let kind = CommitProcessingErrorKind::from(&err);
        assert_eq!(kind, CommitProcessingErrorKind::Other);
    }

    #[test]
    fn commit_processing_error_kind_from_app_error_owned() {
        let err = AppError::IoError(StdIoError::other("something went wrong".to_string()));
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Io);

        let err = Commit::parse("")
            .map_err(AppError::ParseError)
            .expect_err("expected parse error");
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Parse);

        let err = serde_json::from_str::<serde_json::Value>("{ invalid json }")
            .map_err(AppError::from)
            .expect_err("expected JSON parse error");
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Json);

        let err = AppError::FieldError("missing field".into());
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Field);

        let err = AppError::GroupError("no matching group".into());
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Group);

        let err = AppError::GroupError("Skipping commit due to config".into());
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Skipped);

        let err = AppError::UnmatchedCommitsError(1);
        let kind: CommitProcessingErrorKind = err.into();
        assert_eq!(kind, CommitProcessingErrorKind::Other);
    }

    #[test]
    fn commit_processing_error_kind_should_warn_or_not() {
        assert!(CommitProcessingErrorKind::Io.should_warn());
        assert!(CommitProcessingErrorKind::Parse.should_warn());
        assert!(CommitProcessingErrorKind::Json.should_warn());
        assert!(CommitProcessingErrorKind::Field.should_warn());
        assert!(CommitProcessingErrorKind::Group.should_warn());
        assert!(!CommitProcessingErrorKind::Skipped.should_warn());
        assert!(CommitProcessingErrorKind::Other.should_warn());
    }

    #[test]
    fn commit_processing_error_kind_display_is_human_readable() {
        let kind = CommitProcessingErrorKind::Io;
        let s = kind.to_string();
        assert!(!s.is_empty());

        let kind = CommitProcessingErrorKind::Parse;
        let s = kind.to_string();
        assert!(!s.is_empty());

        let kind = CommitProcessingErrorKind::Json;
        let s = kind.to_string();
        assert!(!s.is_empty());

        let kind = CommitProcessingErrorKind::Field;
        let s = kind.to_string();
        assert!(!s.is_empty());

        let kind = CommitProcessingErrorKind::Group;
        let s = kind.to_string();
        assert!(!s.is_empty());

        let kind = CommitProcessingErrorKind::Skipped;
        let s = kind.to_string();
        assert!(!s.is_empty());

        let kind = CommitProcessingErrorKind::Other;
        let s = kind.to_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn summary_record_ok_increments_processed_only() {
        let mut summary = Summary::default();
        summary.record_ok();
        assert_eq!(summary.processed, 1);
        assert!(summary.by_kind.is_empty());
    }

    #[test]
    fn summary_record_err_increments_processed_and_error_kind() {
        let mut summary = Summary::default();
        let err = AppError::FieldError("missing field".into());
        summary.record_err(&err);
        assert_eq!(summary.processed, 1);
        assert_eq!(
            summary.by_kind.get(&CommitProcessingErrorKind::Field),
            Some(&1)
        );
    }

    #[test]
    fn summary_record_err_accumulates_same_kind() {
        let mut summary = Summary::default();
        let err = AppError::FieldError("missing field".into());
        summary.record_err(&err);
        summary.record_err(&err);
        assert_eq!(summary.processed, 2);
        assert_eq!(
            summary.by_kind.get(&CommitProcessingErrorKind::Field),
            Some(&2)
        );
    }
}
