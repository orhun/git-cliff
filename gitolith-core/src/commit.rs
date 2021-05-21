use crate::error::Result;
use git2::Commit as GitCommit;
use git_conventional::Commit as ConventionalCommit;

/// Common commit object that is parsed from a repository.
#[derive(
	Default,
	Debug,
	Clone,
	PartialEq,
	serde_derive::Serialize,
	serde_derive::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
	/// Commit ID.
	pub id:      String,
	/// Commit message including title, description and summary.
	pub message: String,
}

impl<'a> From<GitCommit<'a>> for Commit {
	fn from(commit: GitCommit<'a>) -> Self {
		Self {
			id:      commit.id().to_string()[0..7].to_string(),
			message: commit.message().unwrap_or_default().to_string(),
		}
	}
}

impl Commit {
	/// Returns a conventional commit using the commit message.
	pub fn as_conventional(&self) -> Result<ConventionalCommit> {
		Ok(ConventionalCommit::parse(&self.message)?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn conventional_commit() {
		let test_cases = vec![
			(
				Commit {
					hash:    None,
					message: String::from("test(commit): add test"),
				},
				true,
			),
			(
				Commit {
					hash:    None,
					message: String::from("xyz"),
				},
				false,
			),
		];
		for (commit, is_conventional) in test_cases {
			assert_eq!(is_conventional, commit.as_conventional().is_ok())
		}
	}
}
