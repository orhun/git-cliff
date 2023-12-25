use crate::{
	config::TextProcessor,
	error::{
		Error,
		Result,
	},
};
use serde::Serialize;
use std::collections::{
	HashMap,
	HashSet,
};
use std::error::Error as ErrorImpl;
use tera::{
	ast,
	Context as TeraContext,
	Result as TeraResult,
	Tera,
	Value,
};

/// Wrapper for [`Tera`].
#[derive(Debug)]
pub struct Template {
	tera:          Tera,
	/// Template variables.
	#[cfg_attr(not(feature = "github"), allow(dead_code))]
	pub variables: Vec<String>,
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
		Ok(Self {
			variables: Self::get_template_variables(&tera)?,
			tera,
		})
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

	/// Recursively finds the identifiers from the AST.
	fn find_identifiers(node: &ast::Node, names: &mut HashSet<String>) {
		match node {
			ast::Node::Block(_, block, _) => {
				for node in &block.body {
					Self::find_identifiers(node, names);
				}
			}
			ast::Node::VariableBlock(_, expr) => {
				if let ast::ExprVal::Ident(v) = &expr.val {
					names.insert(v.clone());
				}
			}
			ast::Node::MacroDefinition(_, def, _) => {
				for node in &def.body {
					Self::find_identifiers(node, names);
				}
			}
			ast::Node::FilterSection(_, section, _) => {
				for node in &section.body {
					Self::find_identifiers(node, names);
				}
			}
			ast::Node::Forloop(_, forloop, _) => {
				if let ast::ExprVal::Ident(v) = &forloop.container.val {
					names.insert(v.clone());
				}
				for node in &forloop.body {
					Self::find_identifiers(node, names);
				}
				for node in &forloop.empty_body.clone().unwrap_or_default() {
					Self::find_identifiers(node, names);
				}
			}
			ast::Node::If(cond, _) => {
				for (_, expr, nodes) in &cond.conditions {
					if let ast::ExprVal::Ident(v) = &expr.val {
						names.insert(v.clone());
					}
					for node in nodes {
						Self::find_identifiers(node, names);
					}
				}
				if let Some((_, nodes)) = &cond.otherwise {
					for node in nodes {
						Self::find_identifiers(node, names);
					}
				}
			}
			_ => {}
		}
	}

	/// Returns the variable names that are used in the template.
	fn get_template_variables(tera: &Tera) -> Result<Vec<String>> {
		let mut variables = HashSet::new();
		let ast = &tera.get_template("template")?.ast;
		for node in ast {
			Self::find_identifiers(node, &mut variables);
		}
		Ok(variables.into_iter().collect())
	}

	/// Returns `true` if the template contains GitHub related variables.
	///
	/// Note that this checks the variables starting with "github" and
	/// "commit.github" and ignores "remote.github" values.
	#[cfg(feature = "github")]
	pub(crate) fn contains_github_variable(&self) -> bool {
		self.variables
			.iter()
			.any(|v| v.starts_with("github") || v.starts_with("commit.github"))
	}

	/// Renders the template.
	pub fn render<C: Serialize, T: Serialize, S: Into<String> + Copy>(
		&self,
		context: &C,
		additional_context: Option<&HashMap<S, T>>,
		postprocessors: &[TextProcessor],
	) -> Result<String> {
		let mut context = TeraContext::from_serialize(context)?;
		if let Some(additional_context) = additional_context {
			for (key, value) in additional_context {
				context.insert(*key, &value);
			}
		}
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
	use crate::{
		commit::Commit,
		release::Release,
	};
	use regex::Regex;

	#[test]
	fn render_template() -> Result<()> {
		let template = r#"
		## {{ version }} - <DATE>
		{% for commit in commits %}
		### {{ commit.group }}
		- {{ commit.message | upper_first }}
		{% endfor %}"#;
		let mut template = Template::new(template.to_string(), false)?;
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
					version: Some(String::from("1.0")),
					commits: vec![
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
					previous: None,
					#[cfg(feature = "github")]
					github: crate::github::GitHubReleaseMetadata {
						contributors: vec![],
					},
				},
				Option::<HashMap<&str, String>>::None.as_ref(),
				&[TextProcessor {
					pattern:         Regex::new("<DATE>")
						.expect("failed to compile regex"),
					replace:         Some(String::from("2023")),
					replace_command: None,
				}]
			)?
		);
		template.variables.sort();
		assert_eq!(
			vec![
				String::from("commit.group"),
				String::from("commit.message"),
				String::from("commits"),
				String::from("version"),
			],
			template.variables
		);
		#[cfg(feature = "github")]
		assert!(!template.contains_github_variable());
		Ok(())
	}
}
