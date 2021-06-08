use git_cliff_core::commit::Commit;
use git_cliff_core::config::Config;
use git_cliff_core::error::Result;
use git_cliff_core::release::Release;
use git_cliff_core::template::Template;
use std::io::Write;

/// Changelog generator.
#[derive(Debug)]
pub struct Changelog<'a> {
	releases: Vec<Release<'a>>,
	template: Template,
	config:   &'a Config,
}

impl<'a> Changelog<'a> {
	/// Constructs a new instance.
	pub fn new(releases: Vec<Release<'a>>, config: &'a Config) -> Result<Self> {
		let mut changelog = Self {
			releases,
			template: Template::new(config.changelog.body.to_string())?,
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
		self.releases.iter_mut().for_each(|release| {
			release.commits = release
				.commits
				.iter()
				.filter_map(|commit| {
					match commit.process(
						config.git.commit_parsers.as_ref(),
						config.git.filter_commits,
						config.git.conventional_commits,
					) {
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
		let skip_regex = self.config.git.skip_tags.as_ref();
		self.releases = self
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
					!skip_regex
						.map(|r| {
							let skip_tag = r.is_match(version);
							if skip_tag {
								debug!("Skipping release: {}", version)
							}
							skip_tag
						})
						.unwrap_or_default()
				} else {
					true
				}
			})
			.collect();
	}

	/// Generates the changelog and writes it to the given output.
	pub fn generate<W: Write>(&self, out: &mut W) -> Result<()> {
		if let Some(header) = &self.config.changelog.header {
			writeln!(out, "{}", header)?;
		}
		for release in &self.releases {
			write!(out, "{}", self.template.render(release)?)?;
		}
		if let Some(footer) = &self.config.changelog.footer {
			writeln!(out, "{}", footer)?;
		}
		Ok(())
	}

	/// Generate changelog and prepend it to the given changelog.
	pub fn prepend<W: Write>(
		&self,
		mut changelog: String,
		out: &mut W,
	) -> Result<()> {
		if let Some(header) = &self.config.changelog.header {
			changelog = changelog.replacen(header, "", 1);
		}
		self.generate(out)?;
		write!(out, "{}", changelog)?;
		Ok(())
	}
}
