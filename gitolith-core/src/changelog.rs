use crate::error::Result;
use crate::release::Release;
use tera::{
	Context as TeraContext,
	Tera,
};

/// Changelog generator.
#[derive(Debug)]
pub struct Changelog {
	tera: Tera,
}

impl Changelog {
	/// Constructs a new instance.
	pub fn new(template: String) -> Result<Self> {
		let mut tera = Tera::default();
		tera.add_raw_template("changelog_template", &template)?;
		Ok(Self { tera })
	}

	/// Generates the changelog.
	pub fn generate(&self, release: Release) -> Result<String> {
		Ok(self.tera.render(
			"changelog_template",
			&TeraContext::from_serialize(&release)?,
		)?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit;

	#[test]
	fn changelog_template() -> Result<()> {
		let template = r#"
		## {{ version }}
		{% for commit in commits %}
		### {{ commit.group }}
		- {{ commit.message }}
		{% endfor %}"#;
		let changelog = Changelog::new(template.to_string())?;
		assert_eq!(
			r#"
		## 1.0
		
		### feat
		- add xyz
		
		### fix
		- fix abc
		"#,
			changelog.generate(Release {
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
