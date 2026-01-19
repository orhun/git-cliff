use std::collections::HashMap;
use std::fmt::Display;

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
    pub by_kind: HashMap<CommitProcessingErrorKind, usize>,
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
    // TODO: Implement unit tests.
}
