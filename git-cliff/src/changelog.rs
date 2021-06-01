use git_cliff_core::commit::Commit;
use git_cliff_core::config::ChangelogConfig as Config;
use git_cliff_core::error::Result;
use git_cliff_core::release::ReleaseRoot;
use git_cliff_core::template::Template;
use std::io::Write;

/// Changelog generator.
#[derive(Debug)]
pub struct Changelog<'a> {
	release_root: ReleaseRoot<'a>,
	template:     Template,
	config:       &'a Config,
}

impl<'a> Changelog<'a> {
	/// Constructs a new instance.
	pub fn new(release_root: ReleaseRoot<'a>, config: &'a Config) -> Result<Self> {
		let mut changelog = Self {
			release_root,
			template: Template::new(config.body.to_string())?,
			config,
		};
		changelog.process_commits();
		changelog.process_releases();
		Ok(changelog)
	}

	/// Processes the commits and omits the ones that doesn't match the
	/// criteria set by configuration file.
	fn process_commits(&mut self) {
		let config = &self.config;
		self.release_root.releases.iter_mut().for_each(|release| {
			release.commits = release
				.commits
				.iter()
				.filter_map(|commit| {
					match commit.process(&config.commit_parsers, config.filter_group)
					{
						Ok(commit) => Some(commit),
						Err(e) => {
							debug!("Cannot process commit: {} ({})", commit.id, e);
							None
						}
					}
				})
				.collect::<Vec<Commit>>();
		});
	}

	/// Processes the releases and filters them out based on the configuration.
	fn process_releases(&mut self) {
		self.release_root.releases = self
			.release_root
			.releases
			.clone()
			.into_iter()
			.rev()
			.filter(|release| {
				if release.commits.is_empty() {
					debug!(
						"Release {} doesn't have any commits",
						release
							.version
							.as_ref()
							.cloned()
							.unwrap_or_else(|| String::from("[?]"))
					);
					false
				} else if let Some(version) = &release.version {
					!self.config.skip_tags_regex.is_match(version)
				} else {
					true
				}
			})
			.collect();
	}

	pub fn generate<W: Write>(&self, out: &mut W) -> Result<()> {
		if !self.config.header.is_empty() {
			writeln!(out, "{}", self.config.header)?;
		}
		for release in &self.release_root.releases {
			write!(out, "{}", self.template.render(release)?)?;
		}
		if !self.config.footer.is_empty() {
			writeln!(out, "{}", self.config.footer)?;
		}
		Ok(())
	}
}
