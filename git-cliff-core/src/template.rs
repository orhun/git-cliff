use crate::error::{
	Error,
	Result,
};
use crate::release::Release;
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
	tera: Tera,
}

impl Template {
	/// Constructs a new instance.
	pub fn new(template: String) -> Result<Self> {
		let mut tera = Tera::default();
		if let Err(e) = tera.add_raw_template("template", &template) {
			return if let Some(error_source) = e.source() {
				Err(Error::TemplateParseError(error_source.to_string()))
			} else {
				Err(Error::TemplateError(e))
			};
		}
		tera.register_filter("upper_first", Self::upper_first_filter);
		Ok(Self { tera })
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
	pub fn get_template_variables(&self) -> Result<Vec<String>> {
		let mut variables = HashSet::new();
		let ast = &self.tera.get_template("template")?.ast;
		for node in ast {
			Self::find_identifiers(node, &mut variables);
		}
		Ok(variables.into_iter().collect())
	}

	/// Renders the template.
	pub fn render<T: Serialize, S: Into<String>>(
		&self,
		release: &Release,
		additional_context: Option<HashMap<S, T>>,
	) -> Result<String> {
		let mut context = TeraContext::from_serialize(release)?;
		if let Some(additional_context) = additional_context {
			for (key, value) in additional_context {
				context.insert(key, &value);
			}
		}
		match self.tera.render("template", &context) {
			Ok(v) => Ok(v),
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
		github::GitHubReleaseMetadata,
	};

	#[test]
	fn render_template() -> Result<()> {
		let template = r#"
		## {{ version }}
		{% for commit in commits %}
		### {{ commit.group }}
		- {{ commit.message | upper_first }}
		{% endfor %}"#;
		let template = Template::new(template.to_string())?;
		assert_eq!(
			r#"
		## 1.0
		
		### feat
		- Add xyz
		
		### fix
		- Fix abc
		"#,
			template.render::<String, String>(
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
					github:    GitHubReleaseMetadata {
						contributors: vec![],
					},
				},
				None
			)?
		);
		Ok(())
	}
}
