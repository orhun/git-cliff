use crate::config::CommitParser;
use crate::error::{
	Error as AppError,
	Result,
};
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
	pub conv:    Option<ConventionalCommit<'a>>,
	/// Commit group based on a group parser or its conventional type.
	pub group:   Option<String>,
}

impl<'a> From<&GitCommit<'a>> for Commit<'a> {
	fn from(commit: &GitCommit<'a>) -> Self {
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
			conv: None,
			group: None,
		}
	}

	/// Processes the commit.
	///
	/// * converts commit to a conventional commit
	/// * sets the group for the commit
	pub fn process(&self, parsers: &[CommitParser], filter: bool) -> Result<Self> {
		let commit = self.clone();
		let commit = commit.into_conventional()?;
		let commit = commit.into_grouped(parsers, filter)?;
		Ok(commit)
	}

	/// Returns the commit with its conventional type set.
	pub fn into_conventional(mut self) -> Result<Self> {
		match ConventionalCommit::parse(Box::leak(
			self.message.to_string().into_boxed_str(),
		)) {
			Ok(conv) => {
				self.conv = Some(conv);
				Ok(self)
			}
			Err(e) => Err(AppError::ParseError(e)),
		}
	}

	/// Returns the commit with its group set.
	pub fn into_grouped(
		mut self,
		parsers: &[CommitParser],
		filter: bool,
	) -> Result<Self> {
		for parser in parsers {
			if parser.regex.is_match(&self.message) {
				if parser.skip != Some(true) {
					self.group = parser.group.as_ref().cloned();
					return Ok(self);
				} else {
					return Err(AppError::GroupError);
				}
			}
		}
		if !filter {
			Ok(self)
		} else {
			Err(AppError::GroupError)
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
		match &self.conv {
			Some(conv) => {
				commit.serialize_field("message", conv.description())?;
				commit.serialize_field(
					"group",
					self.group.as_ref().unwrap_or(&conv.type_().to_string()),
				)?;
			}
			None => {
				commit.serialize_field("message", &self.message)?;
				commit.serialize_field("group", &self.group)?;
			}
		}
		commit.end()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use regex::Regex;
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
			assert_eq!(is_conventional, &commit.clone().into_conventional().is_ok())
		}
		let commit = test_cases[0]
			.0
			.clone()
			.into_grouped(
				&[CommitParser {
					regex: Regex::new("test*").unwrap(),
					group: Some(String::from("test_group")),
					skip:  None,
				}],
				false,
			)
			.unwrap();
		assert_eq!(Some(String::from("test_group")), commit.group);
	}
}
