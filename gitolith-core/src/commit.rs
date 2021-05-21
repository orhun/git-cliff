use crate::error::Result;
use git2::Commit as GitCommit;
use git_conventional::Commit as ConventionalCommit;
use serde::ser::{
	Serialize,
	SerializeStruct,
	Serializer,
};

/// Common commit object that is parsed from a repository.
#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit<'a> {
	/// Commit ID.
	pub id:      String,
	/// Commit message including title, description and summary.
	pub message: String,
	/// Conventional commit.
	#[serde(skip_deserializing)]
	pub conv:    ConventionalCommit<'a>,
}

impl<'a> Commit<'a> {
	/// Constructs a new instance.
	pub fn new(commit: GitCommit<'a>) -> Result<Self> {
		let message = commit.message().unwrap_or_default().to_string();
		Ok(Self {
			id:      commit.id().to_string()[0..7].to_string(),
			message: message.to_string(),
			conv:    ConventionalCommit::parse(Box::leak(message.into_boxed_str()))?,
		})
	}
}

impl Serialize for Commit<'_> {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut commit = serializer.serialize_struct("Commit", 3)?;
		commit.serialize_field("id", &self.id)?;
		commit.serialize_field("message", &self.conv.description())?;
		commit.serialize_field("commit_type", &self.conv.type_())?;
		commit.end()
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
