use crate::config::{
	CommitParser,
	GitConfig,
	LinkParser,
	TextProcessor,
};
use crate::error::{
	Error as AppError,
	Result,
};
use git_conventional::{
	Commit as ConventionalCommit,
	Footer as ConventionalFooter,
};
#[cfg(feature = "repo")]
use git2::{
	Commit as GitCommit,
	Signature as CommitSignature,
};
use lazy_regex::{
	Lazy,
	Regex,
	lazy_regex,
};
use serde::ser::{
	SerializeStruct,
	Serializer,
};
use serde::{
	Deserialize,
	Deserializer,
	Serialize,
};
use serde_json::value::Value;

/// Regular expression for matching SHA1 and a following commit message
/// separated by a whitespace.
static SHA1_REGEX: Lazy<Regex> = lazy_regex!(r#"^\b([a-f0-9]{40})\b (.*)$"#);

/// Object representing a link
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Link {
	/// Text of the link.
	pub text: String,
	/// URL of the link
	pub href: String,
}

/// A conventional commit footer.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
struct Footer<'a> {
	/// Token of the footer.
	///
	/// This is the part of the footer preceding the separator. For example, for
	/// the `Signed-off-by: <user.name>` footer, this would be `Signed-off-by`.
	token:     &'a str,
	/// The separator between the footer token and its value.
	///
	/// This is typically either `:` or `#`.
	separator: &'a str,
	/// The value of the footer.
	value:     &'a str,
	/// A flag to signal that the footer describes a breaking change.
	breaking:  bool,
}

impl<'a> From<&'a ConventionalFooter<'a>> for Footer<'a> {
	fn from(footer: &'a ConventionalFooter<'a>) -> Self {
		Self {
			token:     footer.token().as_str(),
			separator: footer.separator().as_str(),
			value:     footer.value(),
			breaking:  footer.breaking(),
		}
	}
}

/// Commit signature that indicates authorship.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Signature {
	/// Name on the signature.
	pub name:      Option<String>,
	/// Email on the signature.
	pub email:     Option<String>,
	/// Time of the signature.
	pub timestamp: i64,
}

#[cfg(feature = "repo")]
impl<'a> From<CommitSignature<'a>> for Signature {
	fn from(signature: CommitSignature<'a>) -> Self {
		Self {
			name:      signature.name().map(String::from),
			email:     signature.email().map(String::from),
			timestamp: signature.when().seconds(),
		}
	}
}

/// Commit range (from..to)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
	/// Full commit SHA the range starts at
	from: String,
	/// Full commit SHA the range ends at
	to:   String,
}

impl Range {
	/// Creates a new [`Range`] from [`crate::commit::Commit`].
	pub fn new(from: &Commit, to: &Commit) -> Self {
		Self {
			from: from.id.clone(),
			to:   to.id.clone(),
		}
	}
}

/// Common commit object that is parsed from a repository.
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
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
	/// Commit author.
	pub author:        Signature,
	/// Committer.
	pub committer:     Signature,
	/// Whether if the commit has two or more parents.
	pub merge_commit:  bool,
	/// Arbitrary data to be used with the `--from-context` CLI option.
	pub extra:         Option<Value>,
	/// Remote metadata of the commit.
	pub remote:        Option<crate::contributor::RemoteContributor>,
	/// GitHub metadata of the commit.
	#[cfg(feature = "github")]
	#[deprecated(note = "Use `remote` field instead")]
	pub github:        crate::contributor::RemoteContributor,
	/// GitLab metadata of the commit.
	#[cfg(feature = "gitlab")]
	#[deprecated(note = "Use `remote` field instead")]
	pub gitlab:        crate::contributor::RemoteContributor,
	/// Gitea metadata of the commit.
	#[cfg(feature = "gitea")]
	#[deprecated(note = "Use `remote` field instead")]
	pub gitea:         crate::contributor::RemoteContributor,
	/// Bitbucket metadata of the commit.
	#[cfg(feature = "bitbucket")]
	#[deprecated(note = "Use `remote` field instead")]
	pub bitbucket:     crate::contributor::RemoteContributor,

	/// Raw message of the normal commit, works as a placeholder for converting
	/// normal commit into conventional commit.
	///
	/// Despite the name, it is not actually a raw message.
	/// In fact, it is pre-processed by [`Commit::preprocess`], and only be
	/// generated when serializing into `context` the first time.
	pub raw_message: Option<String>,
}

impl From<String> for Commit<'_> {
	fn from(message: String) -> Self {
		if let Some(captures) = SHA1_REGEX.captures(&message) {
			if let (Some(id), Some(message)) = (
				captures.get(1).map(|v| v.as_str()),
				captures.get(2).map(|v| v.as_str()),
			) {
				return Commit {
					id: id.to_string(),
					message: message.to_string(),
					..Default::default()
				};
			}
		}
		Commit {
			id: String::new(),
			message,
			..Default::default()
		}
	}
}

#[cfg(feature = "repo")]
impl From<&GitCommit<'_>> for Commit<'_> {
	fn from(commit: &GitCommit<'_>) -> Self {
		Commit {
			id: commit.id().to_string(),
			message: commit.message().unwrap_or_default().trim_end().to_string(),
			author: commit.author().into(),
			committer: commit.committer().into(),
			merge_commit: commit.parent_count() > 1,
			..Default::default()
		}
	}
}

impl Commit<'_> {
	/// Constructs a new instance.
	pub fn new(id: String, message: String) -> Self {
		Self {
			id,
			message,
			..Default::default()
		}
	}

	/// Get raw message for converting into conventional commit.
	pub fn raw_message(&self) -> &str {
		self.raw_message.as_deref().unwrap_or(&self.message)
	}

	/// Processes the commit.
	///
	/// * converts commit to a conventional commit
	/// * sets the group for the commit
	/// * extracts links and generates URLs
	pub fn process(&self, config: &GitConfig) -> Result<Self> {
		let mut commit = self.clone();
		commit = commit.preprocess(&config.commit_preprocessors)?;
		if config.conventional_commits {
			if !config.require_conventional &&
				config.filter_unconventional &&
				!config.split_commits
			{
				commit = commit.into_conventional()?;
			} else if let Ok(conv_commit) = commit.clone().into_conventional() {
				commit = conv_commit;
			}
		}

		commit = commit.parse(
			&config.commit_parsers,
			config.protect_breaking_commits,
			config.filter_commits,
		)?;

		commit = commit.parse_links(&config.link_parsers)?;

		Ok(commit)
	}

	/// Returns the commit with its conventional type set.
	pub fn into_conventional(mut self) -> Result<Self> {
		match ConventionalCommit::parse(Box::leak(
			self.raw_message().to_string().into_boxed_str(),
		)) {
			Ok(conv) => {
				self.conv = Some(conv);
				Ok(self)
			}
			Err(e) => Err(AppError::ParseError(e)),
		}
	}

	/// Preprocesses the commit using [`TextProcessor`]s.
	///
	/// Modifies the commit [`message`] using regex or custom OS command.
	///
	/// [`message`]: Commit::message
	pub fn preprocess(mut self, preprocessors: &[TextProcessor]) -> Result<Self> {
		preprocessors.iter().try_for_each(|preprocessor| {
			preprocessor
				.replace(&mut self.message, vec![("COMMIT_SHA", &self.id)])?;
			Ok::<(), AppError>(())
		})?;
		Ok(self)
	}

	/// States if the commit is skipped in the provided `CommitParser`.
	///
	/// Returns `false` if `protect_breaking_commits` is enabled in the config
	/// and the commit is breaking, or the parser's `skip` field is None or
	/// `false`. Returns `true` otherwise.
	fn skip_commit(&self, parser: &CommitParser, protect_breaking: bool) -> bool {
		parser.skip.unwrap_or(false) &&
			!(self.conv.as_ref().map(|c| c.breaking()).unwrap_or(false) &&
				protect_breaking)
	}

	/// Parses the commit using [`CommitParser`]s.
	///
	/// Sets the [`group`] and [`scope`] of the commit.
	///
	/// [`group`]: Commit::group
	/// [`scope`]: Commit::scope
	pub fn parse(
		mut self,
		parsers: &[CommitParser],
		protect_breaking: bool,
		filter: bool,
	) -> Result<Self> {
		let lookup_context = serde_json::to_value(&self).map_err(|e| {
			AppError::FieldError(format!(
				"failed to convert context into value: {e}",
			))
		})?;
		for parser in parsers {
			let mut regex_checks = Vec::new();
			if let Some(message_regex) = parser.message.as_ref() {
				regex_checks.push((message_regex, self.message.to_string()));
			}
			let body = self
				.conv
				.as_ref()
				.and_then(|v| v.body())
				.map(|v| v.to_string());
			if let Some(body_regex) = parser.body.as_ref() {
				regex_checks.push((body_regex, body.clone().unwrap_or_default()));
			}
			if let (Some(footer_regex), Some(footers)) = (
				parser.footer.as_ref(),
				self.conv.as_ref().map(|v| v.footers()),
			) {
				regex_checks
					.extend(footers.iter().map(|f| (footer_regex, f.to_string())));
			}
			if let (Some(field_name), Some(pattern_regex)) =
				(parser.field.as_ref(), parser.pattern.as_ref())
			{
				let value = if field_name == "body" {
					body.clone()
				} else {
					tera::dotted_pointer(&lookup_context, field_name).and_then(|v| {
						match &v {
							Value::String(s) => Some(s.clone()),
							Value::Number(_) | Value::Bool(_) | Value::Null => {
								Some(v.to_string())
							}
							_ => None,
						}
					})
				};
				match value {
					Some(value) => {
						regex_checks.push((pattern_regex, value));
					}
					None => {
						return Err(AppError::FieldError(format!(
							"field '{field_name}' is missing or has unsupported \
							 type (expected String, Number, Bool, or Null)",
						)));
					}
				}
			}
			if parser.sha.clone().map(|v| v.to_lowercase()).as_deref() ==
				Some(&self.id)
			{
				if self.skip_commit(parser, protect_breaking) {
					return Err(AppError::GroupError(String::from(
						"Skipping commit",
					)));
				} else {
					self.group = parser.group.clone().or(self.group);
					self.scope = parser.scope.clone().or(self.scope);
					self.default_scope =
						parser.default_scope.clone().or(self.default_scope);
					return Ok(self);
				}
			}
			for (regex, text) in regex_checks {
				if regex.is_match(text.trim()) {
					if self.skip_commit(parser, protect_breaking) {
						return Err(AppError::GroupError(String::from(
							"Skipping commit",
						)));
					} else {
						let regex_replace = |mut value: String| {
							for mat in regex.find_iter(&text) {
								value =
									regex.replace(mat.as_str(), value).to_string();
							}
							value
						};
						self.group = parser.group.clone().map(regex_replace);
						self.scope = parser.scope.clone().map(regex_replace);
						self.default_scope.clone_from(&parser.default_scope);
						return Ok(self);
					}
				}
			}
		}
		if filter {
			Err(AppError::GroupError(String::from(
				"Commit does not belong to any group",
			)))
		} else {
			Ok(self)
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

	/// Returns an iterator over this commit's [`Footer`]s, if this is a
	/// conventional commit.
	///
	/// If this commit is not conventional, the returned iterator will be empty.
	fn footers(&self) -> impl Iterator<Item = Footer<'_>> {
		self.conv
			.iter()
			.flat_map(|conv| conv.footers().iter().map(Footer::from))
	}
}

impl Serialize for Commit<'_> {
	#[allow(deprecated)]
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		/// A wrapper to serialize commit footers from an iterator using
		/// `Serializer::collect_seq` without having to allocate in order to
		/// `collect` the footers  into a new to `Vec`.
		struct SerializeFooters<'a>(&'a Commit<'a>);
		impl Serialize for SerializeFooters<'_> {
			fn serialize<S>(
				&self,
				serializer: S,
			) -> std::result::Result<S::Ok, S::Error>
			where
				S: Serializer,
			{
				serializer.collect_seq(self.0.footers())
			}
		}

		let mut commit = serializer.serialize_struct("Commit", 20)?;
		commit.serialize_field("id", &self.id)?;
		if let Some(conv) = &self.conv {
			commit.serialize_field("message", conv.description())?;
			commit.serialize_field("body", &conv.body())?;
			commit.serialize_field("footers", &SerializeFooters(self))?;
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
		} else {
			commit.serialize_field("message", &self.message)?;
			commit.serialize_field("group", &self.group)?;
			commit.serialize_field(
				"scope",
				&self.scope.as_deref().or(self.default_scope.as_deref()),
			)?;
		}

		commit.serialize_field("links", &self.links)?;
		commit.serialize_field("author", &self.author)?;
		commit.serialize_field("committer", &self.committer)?;
		commit.serialize_field("conventional", &self.conv.is_some())?;
		commit.serialize_field("merge_commit", &self.merge_commit)?;
		commit.serialize_field("extra", &self.extra)?;
		#[cfg(feature = "github")]
		commit.serialize_field("github", &self.github)?;
		#[cfg(feature = "gitlab")]
		commit.serialize_field("gitlab", &self.gitlab)?;
		#[cfg(feature = "gitea")]
		commit.serialize_field("gitea", &self.gitea)?;
		#[cfg(feature = "bitbucket")]
		commit.serialize_field("bitbucket", &self.bitbucket)?;
		if let Some(remote) = &self.remote {
			commit.serialize_field("remote", remote)?;
		}
		commit.serialize_field("raw_message", &self.raw_message())?;
		commit.end()
	}
}

/// Deserialize commits into conventional commits if they are convertible.
///
/// Serialized commits cannot be deserialized into commits that have
/// [`Commit::conv`]. Thus, we need to manually convert them using
/// [`Commit::into_conventional`].
///
/// This function is to be used only in [`crate::release::Release::commits`].
pub(crate) fn commits_to_conventional_commits<'de, 'a, D: Deserializer<'de>>(
	deserializer: D,
) -> std::result::Result<Vec<Commit<'a>>, D::Error> {
	let commits = Vec::<Commit<'a>>::deserialize(deserializer)?;
	let commits = commits
		.into_iter()
		.map(|commit| commit.clone().into_conventional().unwrap_or(commit))
		.collect();
	Ok(commits)
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn conventional_commit() -> Result<()> {
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
			assert_eq!(is_conventional, &commit.clone().into_conventional().is_ok());
		}
		let commit = test_cases[0].0.clone().parse(
			&[CommitParser {
				sha:           None,
				message:       Regex::new("test*").ok(),
				body:          None,
				footer:        None,
				group:         Some(String::from("test_group")),
				default_scope: Some(String::from("test_scope")),
				scope:         None,
				skip:          None,
				field:         None,
				pattern:       None,
			}],
			false,
			false,
		)?;
		assert_eq!(Some(String::from("test_group")), commit.group);
		assert_eq!(Some(String::from("test_scope")), commit.default_scope);
		Ok(())
	}

	#[test]
	fn conventional_footers() {
		let cfg = crate::config::GitConfig {
			conventional_commits: true,
			..Default::default()
		};
		let test_cases = vec![
			(
				Commit::new(
					String::from("123123"),
					String::from(
						"test(commit): add test\n\nSigned-off-by: Test User \
						 <test@example.com>",
					),
				),
				vec![Footer {
					token:     "Signed-off-by",
					separator: ":",
					value:     "Test User <test@example.com>",
					breaking:  false,
				}],
			),
			(
				Commit::new(
					String::from("123124"),
					String::from(
						"fix(commit): break stuff\n\nBREAKING CHANGE: This commit \
						 breaks stuff\nSigned-off-by: Test User <test@example.com>",
					),
				),
				vec![
					Footer {
						token:     "BREAKING CHANGE",
						separator: ":",
						value:     "This commit breaks stuff",
						breaking:  true,
					},
					Footer {
						token:     "Signed-off-by",
						separator: ":",
						value:     "Test User <test@example.com>",
						breaking:  false,
					},
				],
			),
		];
		for (commit, footers) in &test_cases {
			let commit = commit.process(&cfg).expect("commit should process");
			assert_eq!(&commit.footers().collect::<Vec<_>>(), footers);
		}
	}

	#[test]
	fn parse_link() -> Result<()> {
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
			assert_eq!(is_conventional, &commit.clone().into_conventional().is_ok());
		}
		let commit = Commit::new(
			String::from("123123"),
			String::from("test(commit): add test\n\nImlement RFC456\n\nFixes: #455"),
		);
		let commit = commit.parse_links(&[
			LinkParser {
				pattern: Regex::new("RFC(\\d+)")?,
				href:    String::from("rfc://$1"),
				text:    None,
			},
			LinkParser {
				pattern: Regex::new("#(\\d+)")?,
				href:    String::from("https://github.com/$1"),
				text:    None,
			},
		])?;
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
		Ok(())
	}

	#[test]
	fn parse_commit() {
		assert_eq!(
			Commit::new(String::new(), String::from("test: no sha1 given")),
			Commit::from(String::from("test: no sha1 given"))
		);
		assert_eq!(
			Commit::new(
				String::from("8f55e69eba6e6ce811ace32bd84cc82215673cb6"),
				String::from("feat: do something")
			),
			Commit::from(String::from(
				"8f55e69eba6e6ce811ace32bd84cc82215673cb6 feat: do something"
			))
		);
		assert_eq!(
			Commit::new(
				String::from("3bdd0e690c4cd5bd00e5201cc8ef3ce3fb235853"),
				String::from("chore: do something")
			),
			Commit::from(String::from(
				"3bdd0e690c4cd5bd00e5201cc8ef3ce3fb235853 chore: do something"
			))
		);
		assert_eq!(
			Commit::new(
				String::new(),
				String::from("thisisinvalidsha1 style: add formatting")
			),
			Commit::from(String::from("thisisinvalidsha1 style: add formatting"))
		);
	}

	#[test]
	fn parse_commit_fields() -> Result<()> {
		let mut commit = Commit::new(
			String::from("8f55e69eba6e6ce811ace32bd84cc82215673cb6"),
			String::from("feat: do something"),
		);

		commit.author = Signature {
			name:      Some("John Doe".to_string()),
			email:     None,
			timestamp: 0x0,
		};

		commit.remote = Some(crate::contributor::RemoteContributor {
			username:      None,
			pr_title:      Some("feat: do something".to_string()),
			pr_number:     None,
			pr_labels:     Vec::new(),
			is_first_time: true,
		});

		let parsed_commit = commit.clone().parse(
			&[CommitParser {
				sha:           None,
				message:       None,
				body:          None,
				footer:        None,
				group:         Some(String::from("Test group")),
				default_scope: None,
				scope:         None,
				skip:          None,
				field:         Some(String::from("author.name")),
				pattern:       Regex::new("John Doe").ok(),
			}],
			false,
			false,
		)?;

		assert_eq!(Some(String::from("Test group")), parsed_commit.group);

		let parsed_commit = commit.clone().parse(
			&[CommitParser {
				sha:           None,
				message:       None,
				body:          None,
				footer:        None,
				group:         Some(String::from("Test group")),
				default_scope: None,
				scope:         None,
				skip:          None,
				field:         Some(String::from("remote.pr_title")),
				pattern:       Regex::new("^feat").ok(),
			}],
			false,
			false,
		)?;

		assert_eq!(Some(String::from("Test group")), parsed_commit.group);

		let parse_result = commit.clone().parse(
			&[CommitParser {
				sha:           None,
				message:       None,
				body:          None,
				footer:        None,
				group:         Some(String::from("Invalid group")),
				default_scope: None,
				scope:         None,
				skip:          None,
				field:         Some(String::from("remote.pr_labels")),
				pattern:       Regex::new(".*").ok(),
			}],
			false,
			false,
		);

		assert!(
			parse_result.is_err(),
			"Expected error when using unsupported field `remote.pr_labels`, but \
			 got Ok"
		);

		Ok(())
	}

	#[test]
	fn commit_sha() -> Result<()> {
		let commit = Commit::new(
			String::from("8f55e69eba6e6ce811ace32bd84cc82215673cb6"),
			String::from("feat: do something"),
		);
		let parsed_commit = commit.clone().parse(
			&[CommitParser {
				sha:           Some(String::from(
					"8f55e69eba6e6ce811ace32bd84cc82215673cb6",
				)),
				message:       None,
				body:          None,
				footer:        None,
				group:         None,
				default_scope: None,
				scope:         None,
				skip:          Some(true),
				field:         None,
				pattern:       None,
			}],
			false,
			false,
		);
		assert!(parsed_commit.is_err());

		let parsed_commit = commit.parse(
			&[CommitParser {
				sha:           Some(String::from(
					"8f55e69eba6e6ce811ace32bd84cc82215673cb6",
				)),
				message:       None,
				body:          None,
				footer:        None,
				group:         Some(String::from("Test group")),
				default_scope: None,
				scope:         None,
				skip:          None,
				field:         None,
				pattern:       None,
			}],
			false,
			false,
		)?;
		assert_eq!(Some(String::from("Test group")), parsed_commit.group);

		Ok(())
	}

	#[test]
	fn field_name_regex() -> Result<()> {
		let commit = Commit {
			message: String::from("feat: do something"),
			author: Signature {
				name:      Some("John Doe".to_string()),
				email:     None,
				timestamp: 0x0,
			},
			..Default::default()
		};
		let parsed_commit = commit.clone().parse(
			&[CommitParser {
				sha:           None,
				message:       None,
				body:          None,
				footer:        None,
				group:         Some(String::from("Test group")),
				default_scope: None,
				scope:         None,
				skip:          None,
				field:         Some(String::from("author.name")),
				pattern:       Regex::new("Something else").ok(),
			}],
			false,
			true,
		);

		assert!(parsed_commit.is_err());

		let parsed_commit = commit.parse(
			&[CommitParser {
				sha:           None,
				message:       None,
				body:          None,
				footer:        None,
				group:         Some(String::from("Test group")),
				default_scope: None,
				scope:         None,
				skip:          None,
				field:         Some(String::from("author.name")),
				pattern:       Regex::new("John Doe").ok(),
			}],
			false,
			false,
		)?;

		assert_eq!(Some(String::from("Test group")), parsed_commit.group);
		Ok(())
	}
}
