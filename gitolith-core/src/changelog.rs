use crate::config::ChangelogConfig;
use crate::error::Result;
use std::path::PathBuf;
use tera::{
	Context as TeraContext,
	Tera,
};

/// Context of a template rendering.
#[derive(Debug, Default, serde_derive::Serialize)]
pub struct Context {
	release_title: String,
}

/// Changelog generator.
#[derive(Debug, Default)]
pub struct Changelog {
	tera:   Tera,
	config: ChangelogConfig,
}

impl Changelog {
	/// Constructs a new instance.
	pub fn new(template: PathBuf, config: ChangelogConfig) -> Result<Self> {
		let mut changelog = Self::default();
		changelog
			.tera
			.add_template_file(template, Some("changelog_template"))?;
		changelog.config = config;
		Ok(changelog)
	}

	/// Generates the changelog.
	pub fn generate(&self) -> Result<String> {
		let context = Context {
			release_title: self.config.unreleased_title.to_string(),
		};
		Ok(self.tera.render(
			"changelog_template",
			&TeraContext::from_serialize(&context)?,
		)?)
	}
}
