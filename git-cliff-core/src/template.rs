use crate::commit::Commit;
use crate::config::TextProcessor;
use crate::error::{
	Error,
	Result,
};
use indexmap::IndexMap;
use itertools::Itertools as _;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error as ErrorImpl;
use tera::{
	Context as TeraContext,
	Result as TeraResult,
	Tera,
	Value,
};

/// Wrapper for [`Tera`].
#[derive(Debug)]
pub struct Template {
	tera: Tera,
}

impl Template {
	/// Constructs a new instance.
	pub fn new(mut template: String, trim: bool) -> Result<Self> {
		if trim {
			template = template
				.lines()
				.map(|v| v.trim())
				.collect::<Vec<&str>>()
				.join("\n")
		}
		let mut tera = Tera::default();
		if let Err(e) = tera.add_raw_template("template", &template) {
			return if let Some(error_source) = e.source() {
				Err(Error::TemplateParseError(error_source.to_string()))
			} else {
				Err(Error::TemplateError(e))
			};
		}
		tera.register_filter("upper_first", Self::upper_first_filter);
		tera.register_filter("commit_groups", Self::commit_groups);
		Ok(Self { tera })
	}

	/// Filter for grouping the commits based on their `group` which is set by
	/// `commit_parsers`.
	fn commit_groups(
		value: &Value,
		_: &HashMap<String, Value>,
	) -> TeraResult<Value> {
		let mut commits =
			tera::try_get_value!("commit_groups", "value", Vec<Commit>, value);

		let groups_ordered_filter = &crate::changelog::COMMIT_GROUPS
			.read()
			.expect("failed to read commit groups");

		commits.retain(|commit| {
			groups_ordered_filter.iter().any(|group| {
				group == commit.group.as_deref().expect("commit must have group")
			})
		});

		let groups = commits
			.into_iter()
			.into_group_map_by(|commit| {
				commit.group.clone().expect("commit must have group")
			})
			.into_iter()
			.sorted_by_key(|(group_name, _)| {
				groups_ordered_filter
					.iter()
					.position(|group| group_name == group)
					.expect("commits have already been filtered to specified groups")
			});

		let mut ordered_groups = IndexMap::new();

		for (group, commits) in groups {
			ordered_groups.insert(group, commits);
		}

		Ok(tera::to_value(ordered_groups)?)
	}

	/// Filter for making the first character of a string uppercase.
	fn upper_first_filter(
		value: &Value,
		_: &HashMap<String, Value>,
	) -> TeraResult<Value> {
		let mut s =
			tera::try_get_value!("upper_first_filter", "value", String, value);
		let mut c = s.chars();
		s = match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		};
		Ok(tera::to_value(&s)?)
	}

	/// Renders the template.
	pub fn render<T: Serialize>(
		&self,
		context: &T,
		postprocessors: &[TextProcessor],
	) -> Result<String> {
		let context = TeraContext::from_serialize(context)?;
		match self.tera.render("template", &context) {
			Ok(mut v) => {
				for postprocessor in postprocessors {
					postprocessor.replace(&mut v, vec![])?;
				}
				Ok(v)
			}
			Err(e) => {
				return if let Some(error_source) = e.source() {
					Err(Error::TemplateRenderError(error_source.to_string()))
				} else {
					Err(Error::TemplateError(e))
				};
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit;
	use crate::release::Release;
	use regex::Regex;

	#[test]
	fn render_template() -> Result<()> {
		let template = r#"
		## {{ version }} - <DATE>
		{% for commit in commits %}
		### {{ commit.group }}
		- {{ commit.message | upper_first }}
		{% endfor %}"#;
		let template = Template::new(template.to_string(), false)?;
		assert_eq!(
			r#"
		## 1.0 - 2023
		
		### feat
		- Add xyz
		
		### fix
		- Fix abc
		"#,
			template.render(
				&Release {
					version:   Some(String::from("1.0")),
					commits:   vec![
						Commit::new(
							String::from("123123"),
							String::from("feat(xyz): add xyz"),
						),
						Commit::new(
							String::from("124124"),
							String::from("fix(abc): fix abc"),
						)
					]
					.into_iter()
					.filter_map(|c| c.into_conventional().ok())
					.collect(),
					commit_id: None,
					timestamp: 0,
					previous:  None,
				},
				&[TextProcessor {
					pattern:         Regex::new("<DATE>")
						.expect("failed to compile regex"),
					replace:         Some(String::from("2023")),
					replace_command: None,
				}]
			)?
		);
		Ok(())
	}
}
