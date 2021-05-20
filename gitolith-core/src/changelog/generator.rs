use crate::changelog::context::{
	Change,
	Context,
};
use crate::config::ChangelogConfig;
use crate::error::Result;
use crate::release::Release;
use std::path::PathBuf;
use tera::{
	Context as TeraContext,
	Tera,
};

/// Changelog generator.
#[derive(Debug)]
pub struct Changelog<'a> {
	tera:   Tera,
	config: &'a ChangelogConfig,
}

impl<'a> Changelog<'a> {
	/// Constructs a new instance.
	pub fn new(template: PathBuf, config: &'a ChangelogConfig) -> Result<Self> {
		let mut tera = Tera::default();
		tera.add_template_file(template, Some("changelog_template"))?;
		Ok(Self { tera, config })
	}

	/// Generates the changelog.
	pub fn generate(&self, release: Release) -> Result<String> {
		let context = Context {
			release_title: release
				.version
				.unwrap_or_else(|| self.config.unreleased_title.to_string()),
			changes:       vec![Change {
				title:   String::from("commits"),
				entries: release.commits,
			}],
		};
		Ok(self.tera.render(
			"changelog_template",
			&TeraContext::from_serialize(&context)?,
		)?)
	}
}
