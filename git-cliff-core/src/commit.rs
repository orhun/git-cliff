use crate::command;
use crate::config::{
	CommitParser,
	CommitPreprocessor,
	GitConfig,
	LinkParser,
};
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
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit<'a> {
	/// Commit ID.
	pub id:            String,
	/// Commit message including title, description and summary.
	pub message:       String,
	/// Conventional commit.
	#[serde(skip_deserializing)]
	pub conv:          Option<ConventionalCommit<'a>>,
	/// Commit group based on a commit parser or its conventional type.
	pub group:         Option<String>,
	/// Default commit scope based on (inherited from) conventional type or a
	/// commit parser.
	pub default_scope: Option<String>,
	/// Commit scope for overriding the default one.
	pub scope:         Option<String>,
	/// A list of links found in the commit
	pub links:         Vec<Link>,
}

/// Object representing a link
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
	/// Text of the link.
	pub text: String,
	/// URL of the link
	pub href: String,
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
			default_scope: None,
			scope: None,
			links: vec![],
		}
	}

	/// Processes the commit.
	///
	/// * converts commit to a conventional commit
	/// * sets the group for the commit
	/// * extacts links and generates URLs
	pub fn process(&self, config: &GitConfig) -> Result<Self> {
		let mut commit = self.clone();
		if let Some(preprocessors) = &config.commit_preprocessors {
			commit = commit.preprocess(preprocessors)?;
		}
		if config.conventional_commits.unwrap_or(true) {
			if config.filter_unconventional.unwrap_or(true) {
				commit = commit.into_conventional()?;
			} else if let Ok(conv_commit) = commit.clone().into_conventional() {
				commit = conv_commit;
			}
		}
		if let Some(parsers) = &config.commit_parsers {
			commit =
				commit.parse(parsers, config.filter_commits.unwrap_or(false))?;
		}
		if let Some(parsers) = &config.link_parsers {
			commit = commit.parse_links(parsers)?;
		}
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

	/// Preprocesses the commit using [`CommitPreprocessor`]s.
	///
	/// Modifies the commit [`message`] using regex or custom OS command.
	///
	/// [`message`]: Commit::message
	pub fn preprocess(
		mut self,
		preprocessors: &[CommitPreprocessor],
	) -> Result<Self> {
		preprocessors.iter().try_for_each(|preprocessor| {
			if let Some(text) = &preprocessor.replace {
				self.message = preprocessor
					.pattern
					.replace_all(&self.message, text)
					.to_string();
			} else if let Some(command) = &preprocessor.replace_command {
				if preprocessor.pattern.is_match(&self.message) {
					self.message = command::run(
						command,
						Some(self.message.to_string()),
						vec![("COMMIT_SHA", &self.id)],
					)?;
				}
			}
			Ok::<(), AppError>(())
		})?;
		Ok(self)
	}

	/// Parses the commit using [`CommitParser`]s.
	///
	/// Sets the [`group`] and [`scope`] of the commit.
	///
	/// [`group`]: Commit::group
	/// [`scope`]: Commit::scope
	pub fn parse(mut self, parsers: &[CommitParser], filter: bool) -> Result<Self> {
		for parser in parsers {
			let mut regex_checks = Vec::new();
			if let Some(message_regex) = parser.message.as_ref() {
				regex_checks.push((message_regex, self.message.to_string()))
			}
			if let (Some(body_regex), Some(body)) = (
				parser.body.as_ref(),
				self.conv.as_ref().and_then(|v| v.body()),
			) {
				regex_checks.push((body_regex, body.to_string()))
			}
			for (regex, text) in regex_checks {
				if regex.is_match(&text) {
					if parser.skip != Some(true) {
						self.group = parser.group.as_ref().cloned();
						self.scope = parser.scope.as_ref().cloned();
						self.default_scope = parser.default_scope.as_ref().cloned();
						return Ok(self);
					} else {
						return Err(AppError::GroupError(String::from(
							"Skipping commit",
						)));
					}
				}
			}
		}
		if !filter {
			Ok(self)
		} else {
			Err(AppError::GroupError(String::from(
				"Commit does not belong to any group",
			)))
		}
	}

	/// Parses the commit using [`LinkParser`]s.
	///
	/// Sets the [`links`] of the commit.
	///
	/// [`links`]: Commit::links
	pub fn parse_links(mut self, parsers: &[LinkParser]) -> Result<Self> {
		for parser in parsers {
			let regex = &parser.pattern;
			let replace = &parser.href;
			for mat in regex.find_iter(&self.message) {
				let m = mat.as_str();
				let text = if let Some(text_replace) = &parser.text {
					regex.replace(m, text_replace).to_string()
				} else {
					m.to_string()
				};
				let href = regex.replace(m, replace);
				self.links.push(Link {
					text,
					href: href.to_string(),
				});
			}
		}
		Ok(self)
	}
}

impl Serialize for Commit<'_> {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut commit = serializer.serialize_struct("Commit", 9)?;
		commit.serialize_field("id", &self.id)?;
		match &self.conv {
			Some(conv) => {
				commit.serialize_field("message", conv.description())?;
				commit.serialize_field("body", &conv.body())?;
				commit.serialize_field(
					"footers",
					&conv
						.footers()
						.to_vec()
						.iter()
						.map(|f| f.value())
						.collect::<Vec<&str>>(),
				)?;
				commit.serialize_field(
					"group",
					self.group.as_ref().unwrap_or(&conv.type_().to_string()),
				)?;
				commit.serialize_field(
					"breaking_description",
					&conv.breaking_description(),
				)?;
				commit.serialize_field("breaking", &conv.breaking())?;
				commit.serialize_field(
					"scope",
					&self
						.scope
						.as_deref()
						.or_else(|| conv.scope().map(|v| v.as_str()))
						.or(self.default_scope.as_deref()),
				)?;
			}
			None => {
				commit.serialize_field("message", &self.message)?;
				commit.serialize_field("group", &self.group)?;
				commit.serialize_field(
					"scope",
					&self
						.scope
						.as_deref()
						.or(self.default_scope.as_deref()),
				)?;
			}
		}
		commit.serialize_field("links", &self.links)?;
		commit.serialize_field("conventional", &self.conv.is_some())?;
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
			.parse(
				&[CommitParser {
					message:       Regex::new("test*").ok(),
					body:          None,
					group:         Some(String::from("test_group")),
					default_scope: Some(String::from("test_scope")),
					scope:         None,
					skip:          None,
				}],
				false,
			)
			.unwrap();
		assert_eq!(Some(String::from("test_group")), commit.group);
		assert_eq!(Some(String::from("test_scope")), commit.default_scope);
	}

	#[test]
	fn parse_link() {
		let test_cases = vec![
			(
				Commit::new(
					String::from("123123"),
					String::from("test(commit): add test\n\nBody with issue #123"),
				),
				true,
			),
			(
				Commit::new(
					String::from("123123"),
					String::from(
						"test(commit): add test\n\nImlement RFC456\n\nFixes: #456",
					),
				),
				true,
			),
		];
		for (commit, is_conventional) in &test_cases {
			assert_eq!(is_conventional, &commit.clone().into_conventional().is_ok())
		}
		let commit = Commit::new(
			String::from("123123"),
			String::from("test(commit): add test\n\nImlement RFC456\n\nFixes: #455"),
		);
		let commit = commit
			.parse_links(&[
				LinkParser {
					pattern: Regex::new("RFC(\\d+)").unwrap(),
					href:    String::from("rfc://$1"),
					text:    None,
				},
				LinkParser {
					pattern: Regex::new("#(\\d+)").unwrap(),
					href:    String::from("https://github.com/$1"),
					text:    None,
				},
			])
			.unwrap();
		assert_eq!(
			vec![
				Link {
					text: String::from("RFC456"),
					href: String::from("rfc://456"),
				},
				Link {
					text: String::from("#455"),
					href: String::from("https://github.com/455"),
				}
			],
			commit.links
		);
	}
}
