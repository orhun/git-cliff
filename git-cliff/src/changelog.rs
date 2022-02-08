use git_cliff_core::commit::Commit;
use git_cliff_core::config::Config;
use git_cliff_core::error::{
	Error,
	Result,
};
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
		let mut template = config
			.changelog
			.body
			.as_deref()
			.unwrap_or_default()
			.to_string();
		if template.is_empty() {
			return Err(Error::ChangelogError(String::from(
				"changelog body cannot be empty",
			)));
		}
		if config.changelog.trim.unwrap_or(true) {
			template = template
				.lines()
				.map(|v| v.trim())
				.collect::<Vec<&str>>()
				.join("\n")
		}
		let mut changelog = Self {
			releases,
			template: Template::new(template)?,
			config,
		};
		changelog.process_commits();
		changelog.process_releases();
		Ok(changelog)
	}

	/// Processes the commits and omits the ones that doesn't match the
	/// criteria set by configuration file.
	fn process_commits(&mut self) {
		debug!("Processing the commits...");
		self.releases.iter_mut().for_each(|release| {
			release.commits = release
				.commits
				.iter()
				.filter_map(|commit| match commit.process(&self.config.git) {
					Ok(commit) => Some(commit),
					Err(e) => {
						trace!(
							"{} - {} ({})",
							commit.id[..7].to_string(),
							e,
							commit.message.lines().next().unwrap_or_default().trim()
						);
						None
					}
				})
				.collect::<Vec<Commit>>();
		});
	}

	/// Processes the releases and filters them out based on the configuration.
	fn process_releases(&mut self) {
		debug!("Processing the releases...");
		let skip_regex = self.config.git.skip_tags.as_ref();
		let mut skipped_tags = Vec::new();
		self.releases = self
			.releases
			.clone()
			.into_iter()
			.rev()
			.filter(|release| {
				if release.commits.is_empty() {
					if let Some(version) = release.version.as_ref().cloned() {
						trace!("Release doesn't have any commits: {}", version);
					}
					false
				} else if let Some(version) = &release.version {
					!skip_regex
						.map(|r| {
							let skip_tag = r.is_match(version);
							if skip_tag {
								skipped_tags.push(version.clone());
								trace!("Skipping release: {}", version)
							}
							skip_tag
						})
						.unwrap_or_default()
				} else {
					true
				}
			})
			.collect();
		for skipped_tag in &skipped_tags {
			if let Some(release_index) = self.releases.iter().position(|release| {
				release
					.previous
					.as_ref()
					.and_then(|release| release.version.as_ref()) ==
					Some(skipped_tag)
			}) {
				if let Some(previous_release) =
					self.releases.get_mut(release_index + 1)
				{
					previous_release.previous = None;
					self.releases[release_index].previous =
						Some(Box::new(previous_release.clone()));
				} else if release_index == self.releases.len() - 1 {
					self.releases[release_index].previous = None;
				}
			}
		}
	}

	/// Generates the changelog and writes it to the given output.
	pub fn generate<W: Write>(&self, out: &mut W) -> Result<()> {
		debug!("Generating changelog...");
		if let Some(header) = &self.config.changelog.header {
			write!(out, "{}", header)?;
		}
		for release in &self.releases {
			write!(out, "{}", self.template.render(release)?)?;
		}
		if let Some(footer) = &self.config.changelog.footer {
			write!(out, "{}", footer)?;
		}
		Ok(())
	}

	/// Generates a changelog and prepends it to the given changelog.
	pub fn prepend<W: Write>(
		&self,
		mut changelog: String,
		out: &mut W,
	) -> Result<()> {
		debug!("Generating changelog and prepending...");
		if let Some(header) = &self.config.changelog.header {
			changelog = changelog.replacen(header, "", 1);
		}
		self.generate(out)?;
		write!(out, "{}", changelog)?;
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use git_cliff_core::config::{
		ChangelogConfig,
		CommitParser,
		GitConfig,
	};
	use git_cliff_core::regex::Regex;
	use pretty_assertions::assert_eq;
	use std::str;
	#[test]
	fn changelog_generator() -> Result<()> {
		let config = Config {
			changelog: ChangelogConfig {
				header: Some(String::from("# Changelog")),
				body:   Some(String::from(
					r#"{% if version %}
				## Release [{{ version }}] - {{ timestamp | date(format="%Y-%m-%d") }}
				({{ commit_id }}){% else %}
				## Unreleased{% endif %}
				{% for group, commits in commits | group_by(attribute="group") %}
				### {{ group }}{% for group, commits in commits | group_by(attribute="scope") %}
				#### {{ group }}{% for commit in commits %}
				- {{ commit.message }}{% endfor %}
				{% endfor %}{% endfor %}"#,
				)),
				footer: Some(String::from("------------")),
				trim:   Some(true),
			},
			git:       GitConfig {
				conventional_commits:  Some(true),
				filter_unconventional: Some(false),
				commit_parsers:        Some(vec![
					CommitParser {
						message:       Regex::new("feat*").ok(),
						body:          None,
						group:         Some(String::from("New features")),
						default_scope: Some(String::from("other")),
						skip:          None,
					},
					CommitParser {
						message:       Regex::new("fix*").ok(),
						body:          None,
						group:         Some(String::from("Bug Fixes")),
						default_scope: None,
						skip:          None,
					},
					CommitParser {
						message:       Regex::new("merge*").ok(),
						body:          None,
						group:         None,
						default_scope: None,
						skip:          Some(true),
					},
					CommitParser {
						message:       Regex::new(".*").ok(),
						body:          None,
						group:         Some(String::from("Other")),
						default_scope: Some(String::from("other")),
						skip:          None,
					},
				]),
				filter_commits:        Some(false),
				tag_pattern:           None,
				skip_tags:             Regex::new("v3.*").ok(),
				ignore_tags:           None,
				date_order:            Some(false),
				sort_commits:          Some(String::from("oldest")),
				link_parsers:          None,
			},
		};
		let test_release = Release {
			version:   Some(String::from("v1.0.0")),
			commits:   vec![
				Commit::new(
					String::from("0bc123"),
					String::from("feat(app): add cool features"),
				),
				Commit::new(
					String::from("000000"),
					String::from("support unconventional commits"),
				),
				Commit::new(
					String::from("0bc123"),
					String::from("feat: support unscoped commits"),
				),
				Commit::new(
					String::from("0werty"),
					String::from("style(ui): make good stuff"),
				),
				Commit::new(
					String::from("0w3rty"),
					String::from("fix(ui): fix more stuff"),
				),
				Commit::new(
					String::from("0jkl12"),
					String::from("chore(app): do nothing"),
				),
			],
			commit_id: Some(String::from("0bc123")),
			timestamp: 50000000,
			previous:  None,
		};
		let releases = vec![
			test_release.clone(),
			Release {
				version: Some(String::from("v3.0.0")),
				commits: vec![Commit::new(
					String::from("n0thin"),
					String::from("feat(xyz): skip commit"),
				)],
				..Release::default()
			},
			Release {
				version:   None,
				commits:   vec![
					Commit::new(
						String::from("abc123"),
						String::from("feat(app): add xyz"),
					),
					Commit::new(
						String::from("abc124"),
						String::from("docs(app): document zyx"),
					),
					Commit::new(String::from("def789"), String::from("merge #4")),
					Commit::new(
						String::from("qwerty"),
						String::from("fix(app): fix abc"),
					),
					Commit::new(
						String::from("hjkl12"),
						String::from("chore(ui): do boring stuff"),
					),
				],
				commit_id: None,
				timestamp: 1000,
				previous:  Some(Box::new(test_release)),
			},
		];
		let changelog = Changelog::new(releases, &config)?;
		let mut out = Vec::new();
		changelog.generate(&mut out)?;
		assert_eq!(
			String::from(
				r#"# Changelog
			## Unreleased

			### Bug Fixes
			#### app
			- fix abc

			### New features
			#### app
			- add xyz

			### Other
			#### app
			- document zyx

			#### ui
			- do boring stuff

			## Release [v1.0.0] - 1971-08-02
			(0bc123)

			### Bug Fixes
			#### ui
			- fix more stuff

			### New features
			#### app
			- add cool features

			#### other
			- support unscoped commits

			### Other
			#### app
			- do nothing

			#### other
			- support unconventional commits

			#### ui
			- make good stuff
			------------"#
			)
			.replace("			", ""),
			str::from_utf8(&out).unwrap()
		);
		Ok(())
	}
}
