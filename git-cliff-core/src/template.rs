use crate::error::Result;
use crate::release::Release;
use std::collections::HashMap;
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
	pub fn new(template: String) -> Result<Self> {
		let mut tera = Tera::default();
		tera.add_raw_template("template", &template)?;
		tera.register_filter("capitalize_first", Self::capitalize_first_filter);
		Ok(Self { tera })
	}

	/// Filter for capitalizing the first character of a string.
	fn capitalize_first_filter(
		value: &Value,
		_: &HashMap<String, Value>,
	) -> TeraResult<Value> {
		let mut s =
			tera::try_get_value!("capitalize_first_filter", "value", String, value);
		let mut c = s.chars();
		s = match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		};
		Ok(tera::to_value(&s)?)
	}

	/// Renders the template.
	pub fn render(&self, release: &Release) -> Result<String> {
		Ok(self
			.tera
			.render("template", &TeraContext::from_serialize(release)?)?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit;

	#[test]
	fn render_template() -> Result<()> {
		let template = r#"
		## {{ version }}
		{% for commit in commits %}
		### {{ commit.group }}
		- {{ commit.message | capitalize_first }}
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
			template.render(Release {
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
			})?
		);
		Ok(())
	}
}
