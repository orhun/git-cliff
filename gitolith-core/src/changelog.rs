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
