use crate::config::GroupParser;
use crate::error::{
	Error as AppError,
	Result,
};
use git2::Commit as GitCommit;
use git_conventional::Commit as ConventionalCommit;
use regex::Regex;
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
	/// Commit group based on a group parser or its conventional type.
	pub group:   Option<String>,
}

impl<'a> From<GitCommit<'a>> for Commit<'a> {
	fn from(commit: GitCommit<'a>) -> Self {
		Self::new(
			commit.id().to_string(),
			commit.message().unwrap_or_default().to_string(),
		)
	}
}

impl Commit<'_> {
	/// Constructs a new instance.
	pub fn new(id: String, message: String) -> Self {
		Self {
			id,
			message,
			conv: ConventionalCommit::default(),
			group: None,
		}
	}

	/// Returns the commit with conventional type.
	pub fn as_conventional(&self) -> Result<Self> {
		match ConventionalCommit::parse(Box::leak(
			self.message.to_string().into_boxed_str(),
		)) {
			Ok(conv) => {
				let mut commit = self.clone();
				commit.conv = conv;
				Ok(commit)
			}
			Err(e) => Err(AppError::ParseError(e)),
		}
	}

	/// Sets the commit group using the given parsers.
	pub fn set_group(&mut self, parsers: &[GroupParser]) {
		for parser in parsers {
			if let Ok(re) = Regex::new(&parser.regex) {
				if re.is_match(&self.message) {
					self.group = Some(parser.group.to_string());
					break;
				}
			}
		}
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
		commit.serialize_field(
			"group",
			self.group
				.as_ref()
				.unwrap_or(&self.conv.type_().to_string()),
		)?;
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
				Commit::new(
					String::from("123123"),
					String::from("test(commit): add test"),
				),
				true,
			),
			(
				Commit::new(String::from("124124"), String::from("xyz")),
				false,
			),
		];
		for (commit, is_conventional) in &test_cases {
			assert_eq!(is_conventional, &commit.as_conventional().is_ok())
		}
		let mut commit = test_cases[0].0.clone();
		commit.set_group(&[GroupParser {
			regex: String::from("test*"),
			group: String::from("test_group"),
		}]);
		assert_eq!(Some(String::from("test_group")), commit.group);
	}
}
