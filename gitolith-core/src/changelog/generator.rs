use crate::changelog::context::Context;
use crate::error::Result;
use crate::release::Release;
use std::path::PathBuf;
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
	pub fn new(template: PathBuf) -> Result<Self> {
		let mut tera = Tera::default();
		tera.add_template_file(template, Some("changelog_template"))?;
		Ok(Self { tera })
	}

	/// Generates the changelog.
	pub fn generate(&self, release: Release) -> Result<String> {
		let context = Context {
			release_title: release.version,
			changes:       release.commits,
		};
		Ok(self.tera.render(
			"changelog_template",
			&TeraContext::from_serialize(&context)?,
		)?)
	}
}
