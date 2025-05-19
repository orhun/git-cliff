use crate::commit::Commit;
use crate::config::{
	Config,
	GitConfig,
};
use crate::error::{
	Error,
	Result,
};
use crate::release::{
	Release,
	Releases,
};
#[cfg(feature = "bitbucket")]
use crate::remote::bitbucket::BitbucketClient;
#[cfg(feature = "gitea")]
use crate::remote::gitea::GiteaClient;
#[cfg(feature = "github")]
use crate::remote::github::GitHubClient;
#[cfg(feature = "gitlab")]
use crate::remote::gitlab::GitLabClient;
use crate::template::Template;
use std::collections::HashMap;
use std::io::{
	Read,
	Write,
};
use std::time::{
	SystemTime,
	UNIX_EPOCH,
};

/// Changelog generator.
#[derive(Debug)]
pub struct Changelog<'a> {
	/// Releases that the changelog will contain.
	pub releases:       Vec<Release<'a>>,
	header_template:    Option<Template>,
	body_template:      Template,
	footer_template:    Option<Template>,
	config:             &'a Config,
	additional_context: HashMap<String, serde_json::Value>,
}

impl<'a> Changelog<'a> {
	/// Constructs a new instance.
	pub fn new(
		releases: Vec<Release<'a>>,
		config: &'a Config,
		range: Option<&str>,
	) -> Result<Self> {
		let mut changelog = Changelog::build(releases, config)?;
		changelog.add_remote_data(range)?;
		changelog.process_commits()?;
		changelog.process_releases();
		Ok(changelog)
	}

	/// Builds a changelog from releases and config.
	fn build(releases: Vec<Release<'a>>, config: &'a Config) -> Result<Self> {
		let trim = config.changelog.trim;
		Ok(Self {
			releases,
			header_template: match &config.changelog.header {
				Some(header) => {
					Some(Template::new("header", header.to_string(), trim)?)
				}
				None => None,
			},
			body_template: get_body_template(config, trim)?,
			footer_template: match &config.changelog.footer {
				Some(footer) => {
					Some(Template::new("footer", footer.to_string(), trim)?)
				}
				None => None,
			},
			config,
			additional_context: HashMap::new(),
		})
	}

	/// Constructs an instance from a serialized context object.
	pub fn from_context<R: Read>(input: &mut R, config: &'a Config) -> Result<Self> {
		Changelog::build(serde_json::from_reader(input)?, config)
	}

	/// Adds a key value pair to the template context.
	///
	/// These values will be used when generating the changelog.
	///
	/// # Errors
	///
	/// This operation fails if the deserialization fails.
	pub fn add_context(
		&mut self,
		key: impl Into<String>,
		value: impl serde::Serialize,
	) -> Result<()> {
		self.additional_context
			.insert(key.into(), serde_json::to_value(value)?);
		Ok(())
	}

	/// Processes a single commit and returns/logs the result.
	fn process_commit(
		commit: &Commit<'a>,
		git_config: &GitConfig,
	) -> Option<Commit<'a>> {
		match commit.process(git_config) {
			Ok(commit) => Some(commit),
			Err(e) => {
				trace!(
					"{} - {} ({})",
					commit.id.chars().take(7).collect::<String>(),
					e,
					commit.message.lines().next().unwrap_or_default().trim()
				);
				None
			}
		}
	}

	/// Checks the commits and returns an error if any unconventional commits
	/// are found.
	fn check_conventional_commits(commits: &Vec<Commit<'a>>) -> Result<()> {
		debug!("Verifying that all commits are conventional.");
		let mut unconventional_count = 0;

		commits.iter().for_each(|commit| {
			if commit.conv.is_none() {
				error!(
					"Commit {id} is not conventional:\n{message}",
					id = &commit.id[..7],
					message = commit
						.message
						.lines()
						.map(|line| { format!("    | {}", line.trim()) })
						.collect::<Vec<String>>()
						.join("\n")
				);
				unconventional_count += 1;
			}
		});

		if unconventional_count > 0 {
			return Err(Error::UnconventionalCommitsError(unconventional_count));
		}

		Ok(())
	}

	fn process_commit_list(
		commits: &mut Vec<Commit<'a>>,
		git_config: &GitConfig,
	) -> Result<()> {
		*commits = commits
			.iter()
			.filter_map(|commit| Self::process_commit(commit, git_config))
			.flat_map(|commit| {
				if git_config.split_commits {
					commit
						.message
						.lines()
						.filter_map(|line| {
							let mut c = commit.clone();
							c.message = line.to_string();
							if c.message.is_empty() {
								None
							} else {
								Self::process_commit(&c, git_config)
							}
						})
						.collect()
				} else {
					vec![commit]
				}
			})
			.collect::<Vec<Commit>>();

		if git_config.require_conventional {
			Self::check_conventional_commits(commits)?;
		}

		Ok(())
	}

	/// Processes the commits and omits the ones that doesn't match the
	/// criteria set by configuration file.
	fn process_commits(&mut self) -> Result<()> {
		debug!("Processing the commits...");
		for release in self.releases.iter_mut() {
			Self::process_commit_list(&mut release.commits, &self.config.git)?;
			for submodule_commits in release.submodule_commits.values_mut() {
				Self::process_commit_list(submodule_commits, &self.config.git)?;
			}
		}
		Ok(())
	}

	/// Processes the releases and filters them out based on the configuration.
	fn process_releases(&mut self) {
		debug!("Processing {} release(s)...", self.releases.len());
		let skip_regex = self.config.git.skip_tags.as_ref();
		let mut skipped_tags = Vec::new();
		self.releases = self
			.releases
			.clone()
			.into_iter()
			.rev()
			.filter(|release| {
				if release.commits.is_empty() {
					if let Some(version) = release.version.clone() {
						trace!("Release doesn't have any commits: {}", version);
					}
					match &release.previous {
						Some(prev_release) if prev_release.commits.is_empty() => {
							self.config.changelog.render_always
						}
						_ => false,
					}
				} else if let Some(version) = &release.version {
					!skip_regex.is_some_and(|r| {
						let skip_tag = r.is_match(version);
						if skip_tag {
							skipped_tags.push(version.clone());
							trace!("Skipping release: {}", version);
						}
						skip_tag
					})
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

	/// Returns the GitHub metadata needed for the changelog.
	///
	/// This function creates a multithread async runtime for handling the
	/// requests. The following are fetched from the GitHub REST API:
	///
	/// - Commits
	/// - Pull requests
	///
	/// Each of these are paginated requests so they are being run in parallel
	/// for speedup.
	///
	/// If no GitHub related variable is used in the template then this function
	/// returns empty vectors.
	#[cfg(feature = "github")]
	fn get_github_metadata(
		&self,
		ref_name: Option<&str>,
	) -> Result<crate::remote::RemoteMetadata> {
		use crate::remote::github;
		if self.config.remote.github.is_custom ||
			self.body_template
				.contains_variable(github::TEMPLATE_VARIABLES) ||
			self.footer_template
				.as_ref()
				.map(|v| v.contains_variable(github::TEMPLATE_VARIABLES))
				.unwrap_or(false)
		{
			debug!("You are using an experimental feature! Please report bugs at <https://git-cliff.org/issues>");
			let github_client =
				GitHubClient::try_from(self.config.remote.github.clone())?;
			info!(
				"{} ({})",
				github::START_FETCHING_MSG,
				self.config.remote.github
			);
			let data = tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()?
				.block_on(async {
					let (commits, pull_requests) = tokio::try_join!(
						github_client.get_commits(ref_name),
						github_client.get_pull_requests(ref_name),
					)?;
					debug!("Number of GitHub commits: {}", commits.len());
					debug!(
						"Number of GitHub pull requests: {}",
						pull_requests.len()
					);
					Ok((commits, pull_requests))
				});
			info!("{}", github::FINISHED_FETCHING_MSG);
			data
		} else {
			Ok((vec![], vec![]))
		}
	}

	/// Returns the GitLab metadata needed for the changelog.
	///
	/// This function creates a multithread async runtime for handling the
	///
	/// requests. The following are fetched from the GitLab REST API:
	///
	/// - Commits
	/// - Merge requests
	///
	/// Each of these are paginated requests so they are being run in parallel
	/// for speedup.
	///
	///
	/// If no GitLab related variable is used in the template then this function
	/// returns empty vectors.
	#[cfg(feature = "gitlab")]
	fn get_gitlab_metadata(
		&self,
		ref_name: Option<&str>,
	) -> Result<crate::remote::RemoteMetadata> {
		use crate::remote::gitlab;
		if self.config.remote.gitlab.is_custom ||
			self.body_template
				.contains_variable(gitlab::TEMPLATE_VARIABLES) ||
			self.footer_template
				.as_ref()
				.map(|v| v.contains_variable(gitlab::TEMPLATE_VARIABLES))
				.unwrap_or(false)
		{
			debug!("You are using an experimental feature! Please report bugs at <https://git-cliff.org/issues>");
			let gitlab_client =
				GitLabClient::try_from(self.config.remote.gitlab.clone())?;
			info!(
				"{} ({})",
				gitlab::START_FETCHING_MSG,
				self.config.remote.gitlab
			);
			let data = tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()?
				.block_on(async {
					// Map repo/owner to gitlab id
					let project_id =
						match tokio::join!(gitlab_client.get_project(ref_name)) {
							(Ok(project),) => project.id,
							(Err(err),) => {
								error!("Failed to lookup project! {}", err);
								return Err(err);
							}
						};
					let (commits, merge_requests) = tokio::try_join!(
						// Send id to these functions
						gitlab_client.get_commits(project_id, ref_name),
						gitlab_client.get_merge_requests(project_id, ref_name),
					)?;
					debug!("Number of GitLab commits: {}", commits.len());
					debug!(
						"Number of GitLab merge requests: {}",
						merge_requests.len()
					);
					Ok((commits, merge_requests))
				});
			info!("{}", gitlab::FINISHED_FETCHING_MSG);
			data
		} else {
			Ok((vec![], vec![]))
		}
	}

	/// Returns the Gitea metadata needed for the changelog.
	///
	/// This function creates a multithread async runtime for handling the
	/// requests. The following are fetched from the GitHub REST API:
	///
	/// - Commits
	/// - Pull requests
	///
	/// Each of these are paginated requests so they are being run in parallel
	/// for speedup.
	///
	/// If no Gitea related variable is used in the template then this function
	/// returns empty vectors.
	#[cfg(feature = "gitea")]
	fn get_gitea_metadata(
		&self,
		ref_name: Option<&str>,
	) -> Result<crate::remote::RemoteMetadata> {
		use crate::remote::gitea;
		if self.config.remote.gitea.is_custom ||
			self.body_template
				.contains_variable(gitea::TEMPLATE_VARIABLES) ||
			self.footer_template
				.as_ref()
				.map(|v| v.contains_variable(gitea::TEMPLATE_VARIABLES))
				.unwrap_or(false)
		{
			debug!("You are using an experimental feature! Please report bugs at <https://git-cliff.org/issues>");
			let gitea_client =
				GiteaClient::try_from(self.config.remote.gitea.clone())?;
			info!(
				"{} ({})",
				gitea::START_FETCHING_MSG,
				self.config.remote.gitea
			);
			let data = tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()?
				.block_on(async {
					let (commits, pull_requests) = tokio::try_join!(
						gitea_client.get_commits(ref_name),
						gitea_client.get_pull_requests(ref_name),
					)?;
					debug!("Number of Gitea commits: {}", commits.len());
					debug!("Number of Gitea pull requests: {}", pull_requests.len());
					Ok((commits, pull_requests))
				});
			info!("{}", gitea::FINISHED_FETCHING_MSG);
			data
		} else {
			Ok((vec![], vec![]))
		}
	}

	/// Returns the Bitbucket metadata needed for the changelog.
	///
	/// This function creates a multithread async runtime for handling the
	///
	/// requests. The following are fetched from the bitbucket REST API:
	///
	/// - Commits
	/// - Pull requests
	///
	/// Each of these are paginated requests so they are being run in parallel
	/// for speedup.
	///
	///
	/// If no bitbucket related variable is used in the template then this
	/// function returns empty vectors.
	#[cfg(feature = "bitbucket")]
	fn get_bitbucket_metadata(
		&self,
		ref_name: Option<&str>,
	) -> Result<crate::remote::RemoteMetadata> {
		use crate::remote::bitbucket;
		if self.config.remote.bitbucket.is_custom ||
			self.body_template
				.contains_variable(bitbucket::TEMPLATE_VARIABLES) ||
			self.footer_template
				.as_ref()
				.map(|v| v.contains_variable(bitbucket::TEMPLATE_VARIABLES))
				.unwrap_or(false)
		{
			debug!("You are using an experimental feature! Please report bugs at <https://git-cliff.org/issues>");
			let bitbucket_client =
				BitbucketClient::try_from(self.config.remote.bitbucket.clone())?;
			info!(
				"{} ({})",
				bitbucket::START_FETCHING_MSG,
				self.config.remote.bitbucket
			);
			let data = tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()?
				.block_on(async {
					let (commits, pull_requests) = tokio::try_join!(
						bitbucket_client.get_commits(ref_name),
						bitbucket_client.get_pull_requests(ref_name)
					)?;
					debug!("Number of Bitbucket commits: {}", commits.len());
					debug!(
						"Number of Bitbucket pull requests: {}",
						pull_requests.len()
					);
					Ok((commits, pull_requests))
				});
			info!("{}", bitbucket::FINISHED_FETCHING_MSG);
			data
		} else {
			Ok((vec![], vec![]))
		}
	}

	/// Adds information about the remote to the template context.
	pub fn add_remote_context(&mut self) -> Result<()> {
		self.additional_context.insert(
			"remote".to_string(),
			serde_json::to_value(self.config.remote.clone())?,
		);
		Ok(())
	}

	/// Adds remote data (e.g. GitHub commits) to the releases.
	pub fn add_remote_data(&mut self, range: Option<&str>) -> Result<()> {
		debug!("Adding remote data...");
		self.add_remote_context()?;

		// Determine the ref at which to fetch remote commits, based on the commit
		// range
		let range_head = range.and_then(|r| r.split("..").last());
		let ref_name = match range_head {
			Some("HEAD") => None,
			other => other,
		};

		#[cfg(feature = "github")]
		let (github_commits, github_pull_requests) = if self.config.remote.github.is_set()
		{
			self.get_github_metadata(ref_name)
				.expect("Could not get github metadata")
		} else {
			(vec![], vec![])
		};
		#[cfg(feature = "gitlab")]
		let (gitlab_commits, gitlab_merge_request) = if self.config.remote.gitlab.is_set()
		{
			self.get_gitlab_metadata(ref_name)
				.expect("Could not get gitlab metadata")
		} else {
			(vec![], vec![])
		};
		#[cfg(feature = "gitea")]
		let (gitea_commits, gitea_merge_request) = if self.config.remote.gitea.is_set() {
			self.get_gitea_metadata(ref_name)
				.expect("Could not get gitea metadata")
		} else {
			(vec![], vec![])
		};
		#[cfg(feature = "bitbucket")]
		let (bitbucket_commits, bitbucket_pull_request) =
			if self.config.remote.bitbucket.is_set() {
				self.get_bitbucket_metadata(ref_name)
					.expect("Could not get bitbucket metadata")
			} else {
				(vec![], vec![])
			};
		#[cfg(feature = "remote")]
		for release in &mut self.releases {
			#[cfg(feature = "github")]
			release.update_github_metadata(
				github_commits.clone(),
				github_pull_requests.clone(),
			)?;
			#[cfg(feature = "gitlab")]
			release.update_gitlab_metadata(
				gitlab_commits.clone(),
				gitlab_merge_request.clone(),
			)?;
			#[cfg(feature = "gitea")]
			release.update_gitea_metadata(
				gitea_commits.clone(),
				gitea_merge_request.clone(),
			)?;
			#[cfg(feature = "bitbucket")]
			release.update_bitbucket_metadata(
				bitbucket_commits.clone(),
				bitbucket_pull_request.clone(),
			)?;
		}
		Ok(())
	}

	/// Increments the version for the unreleased changes based on semver.
	pub fn bump_version(&mut self) -> Result<Option<String>> {
		if let Some(ref mut last_release) = self.releases.iter_mut().next() {
			if last_release.version.is_none() {
				let next_version = last_release
					.calculate_next_version_with_config(&self.config.bump)?;
				debug!("Bumping the version to {next_version}");
				last_release.version = Some(next_version.to_string());
				last_release.timestamp = SystemTime::now()
					.duration_since(UNIX_EPOCH)?
					.as_secs()
					.try_into()?;
				return Ok(Some(next_version));
			}
		}
		Ok(None)
	}

	/// Generates the changelog and writes it to the given output.
	pub fn generate<W: Write + ?Sized>(&self, out: &mut W) -> Result<()> {
		debug!("Generating changelog...");
		let postprocessors = self.config.changelog.postprocessors.clone();

		if let Some(header_template) = &self.header_template {
			let write_result = writeln!(
				out,
				"{}",
				header_template.render(
					&Releases {
						releases: &self.releases,
					},
					Some(&self.additional_context),
					&postprocessors,
				)?
			);
			if let Err(e) = write_result {
				if e.kind() != std::io::ErrorKind::BrokenPipe {
					return Err(e.into());
				}
			}
		}

		for release in &self.releases {
			let write_result = write!(
				out,
				"{}",
				self.body_template.render(
					&release,
					Some(&self.additional_context),
					&postprocessors
				)?
			);
			if let Err(e) = write_result {
				if e.kind() != std::io::ErrorKind::BrokenPipe {
					return Err(e.into());
				}
			}
		}

		if let Some(footer_template) = &self.footer_template {
			let write_result = writeln!(
				out,
				"{}",
				footer_template.render(
					&Releases {
						releases: &self.releases,
					},
					Some(&self.additional_context),
					&postprocessors,
				)?
			);
			if let Err(e) = write_result {
				if e.kind() != std::io::ErrorKind::BrokenPipe {
					return Err(e.into());
				}
			}
		}

		Ok(())
	}

	/// Generates a changelog and prepends it to the given changelog.
	pub fn prepend<W: Write + ?Sized>(
		&self,
		mut changelog: String,
		out: &mut W,
	) -> Result<()> {
		debug!("Generating changelog and prepending...");
		if let Some(header) = &self.config.changelog.header {
			changelog = changelog.replacen(header, "", 1);
		}
		self.generate(out)?;
		write!(out, "{changelog}")?;
		Ok(())
	}

	/// Prints the changelog context to the given output.
	pub fn write_context<W: Write + ?Sized>(&self, out: &mut W) -> Result<()> {
		let output = Releases {
			releases: &self.releases,
		}
		.as_json()?;
		writeln!(out, "{output}")?;
		Ok(())
	}
}

fn get_body_template(config: &Config, trim: bool) -> Result<Template> {
	let template = Template::new("body", config.changelog.body.clone(), trim)?;
	let deprecated_vars = [
		"commit.github",
		"commit.gitea",
		"commit.gitlab",
		"commit.bitbucket",
	];
	if template.contains_variable(&deprecated_vars) {
		warn!(
			"Variables {deprecated_vars:?} are deprecated and will be removed in \
			 the future. Use `commit.remote` instead."
		);
	}
	Ok(template)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::config::{
		Bump,
		ChangelogConfig,
		CommitParser,
		Remote,
		RemoteConfig,
		TextProcessor,
	};
	use pretty_assertions::assert_eq;
	use regex::Regex;
	use std::str;

	fn get_test_data() -> (Config, Vec<Release<'static>>) {
		let config = Config {
			changelog: ChangelogConfig {
				header:         Some(String::from("# Changelog")),
				body:           String::from(
					r#"{% if version %}
				## Release [{{ version }}] - {{ timestamp | date(format="%Y-%m-%d") }} - ({{ repository }})
				{% if commit_id %}({{ commit_id }}){% endif %}{% else %}
				## Unreleased{% endif %}
				{% for group, commits in commits | group_by(attribute="group") %}
				### {{ group }}{% for group, commits in commits | group_by(attribute="scope") %}
				#### {{ group }}{% for commit in commits %}
				- {{ commit.message }}{% endfor %}
				{% endfor %}{% endfor %}"#,
				),
				footer:         Some(String::from(
					r#"-- total releases: {{ releases | length }} --"#,
				)),
				trim:           true,
				postprocessors: vec![TextProcessor {
					pattern:         Regex::new("boring")
						.expect("failed to compile regex"),
					replace:         Some(String::from("exciting")),
					replace_command: None,
				}],
				render_always:  false,
				output:         None,
			},
			git:       GitConfig {
				conventional_commits:     true,
				require_conventional:     false,
				filter_unconventional:    false,
				split_commits:            false,
				commit_preprocessors:     vec![TextProcessor {
					pattern:         Regex::new("<preprocess>")
						.expect("failed to compile regex"),
					replace:         Some(String::from(
						"this commit is preprocessed",
					)),
					replace_command: None,
				}],
				commit_parsers:           vec![
					CommitParser {
						sha:           Some(String::from("tea")),
						message:       None,
						body:          None,
						footer:        None,
						group:         Some(String::from("I love tea")),
						default_scope: None,
						scope:         None,
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           Some(String::from("coffee")),
						message:       None,
						body:          None,
						footer:        None,
						group:         None,
						default_scope: None,
						scope:         None,
						skip:          Some(true),
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           Some(String::from("coffee2")),
						message:       None,
						body:          None,
						footer:        None,
						group:         None,
						default_scope: None,
						scope:         None,
						skip:          Some(true),
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new(r".*merge.*").ok(),
						body:          None,
						footer:        None,
						group:         None,
						default_scope: None,
						scope:         None,
						skip:          Some(true),
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new("feat*").ok(),
						body:          None,
						footer:        None,
						group:         Some(String::from("New features")),
						default_scope: Some(String::from("other")),
						scope:         None,
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new("^fix*").ok(),
						body:          None,
						footer:        None,
						group:         Some(String::from("Bug Fixes")),
						default_scope: None,
						scope:         None,
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new("doc:").ok(),
						body:          None,
						footer:        None,
						group:         Some(String::from("Documentation")),
						default_scope: None,
						scope:         Some(String::from("documentation")),
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new("docs:").ok(),
						body:          None,
						footer:        None,
						group:         Some(String::from("Documentation")),
						default_scope: None,
						scope:         Some(String::from("documentation")),
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new(r"match\((.*)\):.*").ok(),
						body:          None,
						footer:        None,
						group:         Some(String::from("Matched ($1)")),
						default_scope: None,
						scope:         None,
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       None,
						body:          None,
						footer:        Regex::new("Footer:.*").ok(),
						group:         Some(String::from("Footer")),
						default_scope: None,
						scope:         Some(String::from("footer")),
						skip:          None,
						field:         None,
						pattern:       None,
					},
					CommitParser {
						sha:           None,
						message:       Regex::new(".*").ok(),
						body:          None,
						footer:        None,
						group:         Some(String::from("Other")),
						default_scope: Some(String::from("other")),
						scope:         None,
						skip:          None,
						field:         None,
						pattern:       None,
					},
				],
				protect_breaking_commits: false,
				filter_commits:           false,
				tag_pattern:              None,
				skip_tags:                Regex::new("v3.*").ok(),
				ignore_tags:              None,
				count_tags:               None,
				use_branch_tags:          false,
				topo_order:               false,
				topo_order_commits:       true,
				sort_commits:             String::from("oldest"),
				link_parsers:             [].to_vec(),
				limit_commits:            None,
				recurse_submodules:       None,
			},
			remote:    RemoteConfig {
				github:    Remote {
					owner:      String::from("coolguy"),
					repo:       String::from("awesome"),
					token:      None,
					is_custom:  false,
					api_url:    None,
					native_tls: None,
				},
				gitlab:    Remote {
					owner:      String::from("coolguy"),
					repo:       String::from("awesome"),
					token:      None,
					is_custom:  false,
					api_url:    None,
					native_tls: None,
				},
				gitea:     Remote {
					owner:      String::from("coolguy"),
					repo:       String::from("awesome"),
					token:      None,
					is_custom:  false,
					api_url:    None,
					native_tls: None,
				},
				bitbucket: Remote {
					owner:      String::from("coolguy"),
					repo:       String::from("awesome"),
					token:      None,
					is_custom:  false,
					api_url:    None,
					native_tls: None,
				},
			},
			bump:      Bump::default(),
		};
		let test_release = Release {
			version: Some(String::from("v1.0.0")),
			message: None,
			extra: None,
			commits: vec![
				Commit::new(
					String::from("coffee"),
					String::from("revert(app): skip this commit"),
				),
				Commit::new(
					String::from("tea"),
					String::from("feat(app): damn right"),
				),
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
					String::from("qw3rty"),
					String::from("doc: update docs"),
				),
				Commit::new(
					String::from("0bc123"),
					String::from("docs: add some documentation"),
				),
				Commit::new(
					String::from("0jkl12"),
					String::from("chore(app): do nothing"),
				),
				Commit::new(
					String::from("qwerty"),
					String::from("chore: <preprocess>"),
				),
				Commit::new(
					String::from("qwertz"),
					String::from("feat!: support breaking commits"),
				),
				Commit::new(
					String::from("qwert0"),
					String::from("match(group): support regex-replace for groups"),
				),
				Commit::new(
					String::from("coffee"),
					String::from("revert(app): skip this commit"),
				),
				Commit::new(
					String::from("footer"),
					String::from("misc: use footer\n\nFooter: footer text"),
				),
			],
			commit_range: None,
			commit_id: Some(String::from("0bc123")),
			timestamp: 50000000,
			previous: None,
			repository: Some(String::from("/root/repo")),
			submodule_commits: HashMap::from([(
				String::from("submodule_one"),
				vec![
					Commit::new(
						String::from("sub0jkl12"),
						String::from("chore(app): submodule_one do nothing"),
					),
					Commit::new(
						String::from("subqwerty"),
						String::from("chore: submodule_one <preprocess>"),
					),
					Commit::new(
						String::from("subqwertz"),
						String::from(
							"feat!: submodule_one support breaking commits",
						),
					),
					Commit::new(
						String::from("subqwert0"),
						String::from(
							"match(group): submodule_one support regex-replace for \
							 groups",
						),
					),
				],
			)]),
			#[cfg(feature = "github")]
			github: crate::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "gitlab")]
			gitlab: crate::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "gitea")]
			gitea: crate::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "bitbucket")]
			bitbucket: crate::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
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
				version: None,
				message: None,
				extra: None,
				commits: vec![
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
						String::from("dev063"),
						String::from("feat(app)!: merge #5"),
					),
					Commit::new(
						String::from("qwerty"),
						String::from("fix(app): fix abc"),
					),
					Commit::new(
						String::from("hjkl12"),
						String::from("chore(ui): do boring stuff"),
					),
					Commit::new(
						String::from("coffee2"),
						String::from("revert(app): skip this commit"),
					),
				],
				commit_range: None,
				commit_id: None,
				timestamp: 1000,
				previous: Some(Box::new(test_release)),
				repository: Some(String::from("/root/repo")),
				submodule_commits: HashMap::from([
					(String::from("submodule_one"), vec![
						Commit::new(
							String::from("def349"),
							String::from("sub_one merge #4"),
						),
						Commit::new(
							String::from("da8912"),
							String::from("sub_one merge #5"),
						),
					]),
					(String::from("submodule_two"), vec![Commit::new(
						String::from("ab76ef"),
						String::from("sub_two bump"),
					)]),
				]),
				#[cfg(feature = "github")]
				github: crate::remote::RemoteReleaseMetadata {
					contributors: vec![],
				},
				#[cfg(feature = "gitlab")]
				gitlab: crate::remote::RemoteReleaseMetadata {
					contributors: vec![],
				},
				#[cfg(feature = "gitea")]
				gitea: crate::remote::RemoteReleaseMetadata {
					contributors: vec![],
				},
				#[cfg(feature = "bitbucket")]
				bitbucket: crate::remote::RemoteReleaseMetadata {
					contributors: vec![],
				},
			},
		];
		(config, releases)
	}

	#[test]
	fn changelog_generator() -> Result<()> {
		let (config, releases) = get_test_data();
		let mut changelog = Changelog::new(releases, &config, None)?;
		changelog.bump_version()?;
		changelog.releases[0].timestamp = 0;
		let mut out = Vec::new();
		changelog.generate(&mut out)?;
		assert_eq!(
			String::from(
				r#"# Changelog

			## Release [v1.1.0] - 1970-01-01 - (/root/repo)


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
			- do exciting stuff

			## Release [v1.0.0] - 1971-08-02 - (/root/repo)
			(0bc123)

			### Bug Fixes
			#### ui
			- fix more stuff

			### Documentation
			#### documentation
			- update docs
			- add some documentation

			### Footer
			#### footer
			- use footer

			### I love tea
			#### app
			- damn right

			### Matched (group)
			#### group
			- support regex-replace for groups

			### New features
			#### app
			- add cool features

			#### other
			- support unscoped commits
			- support breaking commits

			### Other
			#### app
			- do nothing

			#### other
			- support unconventional commits
			- this commit is preprocessed

			#### ui
			- make good stuff
			-- total releases: 2 --
			"#
			)
			.replace("			", ""),
			str::from_utf8(&out).unwrap_or_default()
		);
		Ok(())
	}

	#[test]
	fn changelog_generator_split_commits() -> Result<()> {
		let (mut config, mut releases) = get_test_data();
		config.git.split_commits = true;
		config.git.filter_unconventional = false;
		config.git.protect_breaking_commits = true;

		for parser in config
			.git
			.commit_parsers
			.iter_mut()
			.filter(|p| p.footer.is_some())
		{
			parser.skip = Some(true);
		}

		releases[0].commits.push(Commit::new(
			String::from("0bc123"),
			String::from(
				"feat(app): add some more cool features
feat(app): even more features
feat(app): feature #3
",
			),
		));
		releases[0].commits.push(Commit::new(
			String::from("003934"),
			String::from(
				"feat: add awesome stuff
fix(backend): fix awesome stuff
style: make awesome stuff look better
",
			),
		));
		releases[2].commits.push(Commit::new(
			String::from("123abc"),
			String::from(
				"chore(deps): bump some deps

chore(deps): bump some more deps
chore(deps): fix broken deps
",
			),
		));
		let changelog = Changelog::new(releases, &config, None)?;
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

			#### deps
			- bump some deps
			- bump some more deps
			- fix broken deps

			#### ui
			- do exciting stuff

			### feat
			#### app
			- merge #5

			## Release [v1.0.0] - 1971-08-02 - (/root/repo)
			(0bc123)

			### Bug Fixes
			#### backend
			- fix awesome stuff

			#### ui
			- fix more stuff

			### Documentation
			#### documentation
			- update docs
			- add some documentation

			### I love tea
			#### app
			- damn right

			### Matched (group)
			#### group
			- support regex-replace for groups

			### New features
			#### app
			- add cool features
			- add some more cool features
			- even more features
			- feature #3

			#### other
			- support unscoped commits
			- support breaking commits
			- add awesome stuff

			### Other
			#### app
			- do nothing

			#### other
			- support unconventional commits
			- this commit is preprocessed
			- make awesome stuff look better

			#### ui
			- make good stuff
			-- total releases: 2 --
			"#
			)
			.replace("			", ""),
			str::from_utf8(&out).unwrap_or_default()
		);
		Ok(())
	}

	#[test]
	fn changelog_adds_additional_context() -> Result<()> {
		let (mut config, releases) = get_test_data();
		// add `{{ custom_field }}` to the template
		config.changelog.body = r#"{% if version %}
				## {{ custom_field }} [{{ version }}] - {{ timestamp | date(format="%Y-%m-%d") }}
				{% if commit_id %}({{ commit_id }}){% endif %}{% else %}
				## Unreleased{% endif %}
				{% for group, commits in commits | group_by(attribute="group") %}
				### {{ group }}{% for group, commits in commits | group_by(attribute="scope") %}
				#### {{ group }}{% for commit in commits %}
				- {{ commit.message }}{% endfor %}
				{% endfor %}{% endfor %}"#
			.to_string();
		let mut changelog = Changelog::new(releases, &config, None)?;
		changelog.add_context("custom_field", "Hello")?;
		let mut out = Vec::new();
		changelog.generate(&mut out)?;
		expect_test::expect![[r#"
    # Changelog

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
    - do exciting stuff

    ## Hello [v1.0.0] - 1971-08-02
    (0bc123)

    ### Bug Fixes
    #### ui
    - fix more stuff

    ### Documentation
    #### documentation
    - update docs
    - add some documentation

    ### Footer
    #### footer
    - use footer

    ### I love tea
    #### app
    - damn right

    ### Matched (group)
    #### group
    - support regex-replace for groups

    ### New features
    #### app
    - add cool features

    #### other
    - support unscoped commits
    - support breaking commits

    ### Other
    #### app
    - do nothing

    #### other
    - support unconventional commits
    - this commit is preprocessed

    #### ui
    - make good stuff
    -- total releases: 2 --
"#]]
		.assert_eq(str::from_utf8(&out).unwrap_or_default());
		Ok(())
	}
}
