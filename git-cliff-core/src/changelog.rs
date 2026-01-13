use std::collections::HashMap;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::commit::Commit;
use crate::config::{Config, GitConfig};
use crate::error::{Error, Result};
use crate::release::{Release, Releases};
#[cfg(feature = "azure_devops")]
use crate::remote::azure_devops::AzureDevOpsClient;
#[cfg(feature = "bitbucket")]
use crate::remote::bitbucket::BitbucketClient;
#[cfg(feature = "gitea")]
use crate::remote::gitea::GiteaClient;
#[cfg(feature = "github")]
use crate::remote::github::GitHubClient;
#[cfg(feature = "gitlab")]
use crate::remote::gitlab::GitLabClient;
use crate::summary::Summary;
use crate::template::Template;

/// Changelog generator.
#[derive(Debug)]
pub struct Changelog<'a> {
    /// Releases that the changelog will contain.
    pub releases: Vec<Release<'a>>,
    /// Configuration used for generating the changelog.
    pub config: Config,
    header_template: Option<Template>,
    body_template: Template,
    footer_template: Option<Template>,
    additional_context: HashMap<String, serde_json::Value>,
}

impl<'a> Changelog<'a> {
    /// Constructs a new instance.
    pub fn new(releases: Vec<Release<'a>>, config: Config, range: Option<&str>) -> Result<Self> {
        let is_offline = config.remote.offline;
        let mut changelog = Changelog::build(releases, config)?;

        // Always add context, but only add data if we are running in online mode.
        changelog.add_remote_context()?;
        if !is_offline {
            changelog.add_remote_data(range)?;
        }

        changelog.process_commits()?;
        changelog.process_releases();
        Ok(changelog)
    }

    /// Builds a changelog from releases and config.
    fn build(releases: Vec<Release<'a>>, config: Config) -> Result<Self> {
        let trim = config.changelog.trim;
        Ok(Self {
            releases,
            header_template: match &config.changelog.header {
                Some(header) => Some(Template::new("header", header.clone(), trim)?),
                None => None,
            },
            body_template: get_body_template(&config, trim)?,
            footer_template: match &config.changelog.footer {
                Some(footer) => Some(Template::new("footer", footer.clone(), trim)?),
                None => None,
            },
            config,
            additional_context: HashMap::new(),
        })
    }

    /// Constructs an instance from a serialized context object.
    pub fn from_context<R: Read>(input: &mut R, config: Config) -> Result<Self> {
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
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip_all,
            fields(id = commit.id)
        )
    )]
    fn process_commit(
        commit: &Commit<'a>,
        git_config: &GitConfig,
        summary: &mut Summary,
    ) -> Option<Commit<'a>> {
        match commit.process(git_config) {
            Ok(commit) => {
                summary.record_ok();
                Some(commit)
            }
            Err(e) => {
                summary.record_err(&e);
                let short_id = commit.id.chars().take(7).collect::<String>();
                let summary = commit.message.lines().next().unwrap_or_default().trim();
                log::trace!("{short_id} - {e} ({summary})");
                None
            }
        }
    }

    /// Checks the commits and returns an error if any unconventional commits
    /// are found.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip_all,
            fields(commits = commits.len())
        )
    )]
    fn check_conventional_commits(commits: &Vec<Commit<'a>>) -> Result<()> {
        log::debug!("Verifying that all commits are conventional");
        let mut unconventional_count = 0;
        commits.iter().for_each(|commit| {
            if commit.conv.is_none() {
                log::error!(
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

    /// Checks the commits and returns an error if any commits are not matched
    /// by any commit parser.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip_all,
            fields(commits = commits.len())
        )
    )]
    fn check_unmatched_commits(commits: &Vec<Commit<'a>>) -> Result<()> {
        log::debug!("Verifying that no commits are unmatched by commit parsers");
        let mut unmatched_count = 0;
        commits.iter().for_each(|commit| {
            let is_unmatched = commit.group.is_none();
            if is_unmatched {
                log::error!(
                    "Commit {id} was not matched by any commit parser:\n{message}",
                    id = &commit.id[..7],
                    message = commit
                        .message
                        .lines()
                        .map(|line| { format!("    | {}", line.trim()) })
                        .collect::<Vec<String>>()
                        .join("\n")
                );
                unmatched_count += 1;
            }
        });

        if unmatched_count > 0 {
            return Err(Error::UnmatchedCommitsError(unmatched_count));
        }

        Ok(())
    }

    /// Processes a commit list by applying parsing, splitting, and validation rules.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip_all,
            fields(commits = commits.len())
        )
    )]
    fn process_commit_list(
        commits: &mut Vec<Commit<'a>>,
        git_config: &GitConfig,
        summary: &mut Summary,
    ) -> Result<()> {
        let mut processed = Vec::new();
        for commit in commits.iter() {
            if let Some(commit) = Self::process_commit(commit, git_config, summary) {
                if git_config.split_commits {
                    for line in commit.message.lines() {
                        let mut c = commit.clone();
                        c.message = line.to_string();
                        c.links.clear();
                        if c.message.is_empty() {
                            continue;
                        }
                        if let Some(c) = Self::process_commit(&c, git_config, summary) {
                            processed.push(c);
                        }
                    }
                } else {
                    processed.push(commit);
                }
            }
        }
        *commits = processed;

        if git_config.require_conventional {
            Self::check_conventional_commits(commits)?;
        }

        if git_config.fail_on_unmatched_commit {
            Self::check_unmatched_commits(commits)?;
        }

        Ok(())
    }

    /// Processes the commits and omits the ones that doesn't match the
    /// criteria set by configuration file.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip_all,
            fields(releases = self.releases.len())
        )
    )]
    fn process_commits(&mut self) -> Result<()> {
        log::debug!("Processing the commits");

        let mut summary = Summary::default();
        for release in self.releases.iter_mut() {
            Self::process_commit_list(&mut release.commits, &self.config.git, &mut summary)?;
            for submodule_commits in release.submodule_commits.values_mut() {
                Self::process_commit_list(submodule_commits, &self.config.git, &mut summary)?;
            }
        }

        log::debug!(
            "Processed {} commit(s) in total (`split_commits` option may cause duplicates)",
            summary.processed
        );
        for (&kind, &count) in &summary.by_kind {
            if count == 0 {
                continue;
            }
            let message = format!(
                "{count} commit(s) were skipped due to {kind}(s) (run with `-vv` for details)",
            );
            if kind.should_warn() {
                log::warn!("{message}");
            } else {
                log::debug!("{message}");
            }
        }

        Ok(())
    }

    /// Processes the releases and filters them out based on the configuration.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip_all,
            fields(releases = self.releases.len())
        )
    )]
    fn process_releases(&mut self) {
        log::debug!("Processing {} release(s)", self.releases.len());
        let skip_regex = self.config.git.skip_tags.as_ref();
        let mut skipped_tags = Vec::new();
        self.releases = self
            .releases
            .clone()
            .into_iter()
            .rev()
            .filter(|release| {
                if let Some(version) = &release.version {
                    if skip_regex.is_some_and(|r| r.is_match(version)) {
                        skipped_tags.push(version.clone());
                        log::debug!("Skipping release: {version}");
                        return false;
                    }
                }
                if release.commits.is_empty() {
                    if let Some(version) = release.version.clone() {
                        log::debug!("Release doesn't have any commits: {version}");
                    }
                    match &release.previous {
                        Some(prev_release) if prev_release.commits.is_empty() => {
                            return self.config.changelog.render_always;
                        }
                        _ => return false,
                    }
                }
                true
            })
            .map(Release::with_statistics)
            .collect();
        for skipped_tag in &skipped_tags {
            if let Some(release_index) = self.releases.iter().position(|release| {
                release
                    .previous
                    .as_ref()
                    .and_then(|release| release.version.as_ref()) ==
                    Some(skipped_tag)
            }) {
                if let Some(previous_release) = self.releases.get_mut(release_index + 1) {
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn get_github_metadata(&self, ref_name: Option<&str>) -> Result<crate::remote::RemoteMetadata> {
        use crate::remote::github;
        if self.config.remote.github.is_custom ||
            self.body_template
                .contains_variable(github::TEMPLATE_VARIABLES) ||
            self.footer_template
                .as_ref()
                .is_some_and(|v| v.contains_variable(github::TEMPLATE_VARIABLES))
        {
            let github_client = GitHubClient::try_from(self.config.remote.github.clone())?;
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    let (commits, pull_requests) = tokio::try_join!(
                        github_client.get_commits(ref_name),
                        github_client.get_pull_requests(),
                    )?;
                    log::debug!("Number of GitHub commits: {}", commits.len());
                    log::debug!("Number of GitHub pull requests: {}", pull_requests.len());
                    Ok((commits, pull_requests))
                })
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn get_gitlab_metadata(&self, ref_name: Option<&str>) -> Result<crate::remote::RemoteMetadata> {
        use crate::remote::gitlab;
        if self.config.remote.gitlab.is_custom ||
            self.body_template
                .contains_variable(gitlab::TEMPLATE_VARIABLES) ||
            self.footer_template
                .as_ref()
                .is_some_and(|v| v.contains_variable(gitlab::TEMPLATE_VARIABLES))
        {
            let gitlab_client = GitLabClient::try_from(self.config.remote.gitlab.clone())?;
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    // Map repo/owner to gitlab id
                    let project_id = match tokio::join!(gitlab_client.get_project()) {
                        (Ok(project),) => project.id,
                        (Err(err),) => {
                            log::error!("Failed to lookup project: {err}");
                            return Err(err);
                        }
                    };
                    let (commits, merge_requests) = tokio::try_join!(
                        // Send id to these functions
                        gitlab_client.get_commits(
                            project_id.expect("Project id is required for git-cliff semantics"),
                            ref_name
                        ),
                        gitlab_client.get_pull_requests(
                            project_id.expect("Project id is required for git-cliff semantics")
                        ),
                    )?;
                    log::debug!("Number of GitLab commits: {}", commits.len());
                    log::debug!("Number of GitLab merge requests: {}", merge_requests.len());
                    Ok((commits, merge_requests))
                })
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn get_gitea_metadata(&self, ref_name: Option<&str>) -> Result<crate::remote::RemoteMetadata> {
        use crate::remote::gitea;
        if self.config.remote.gitea.is_custom ||
            self.body_template
                .contains_variable(gitea::TEMPLATE_VARIABLES) ||
            self.footer_template
                .as_ref()
                .is_some_and(|v| v.contains_variable(gitea::TEMPLATE_VARIABLES))
        {
            let gitea_client = GiteaClient::try_from(self.config.remote.gitea.clone())?;
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    let (commits, pull_requests) = tokio::try_join!(
                        gitea_client.get_commits(ref_name),
                        gitea_client.get_pull_requests(),
                    )?;
                    log::debug!("Number of Gitea commits: {}", commits.len());
                    log::debug!("Number of Gitea pull requests: {}", pull_requests.len());
                    Ok((commits, pull_requests))
                })
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
                .is_some_and(|v| v.contains_variable(bitbucket::TEMPLATE_VARIABLES))
        {
            let bitbucket_client = BitbucketClient::try_from(self.config.remote.bitbucket.clone())?;
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    let (commits, pull_requests) = tokio::try_join!(
                        bitbucket_client.get_commits(ref_name),
                        bitbucket_client.get_pull_requests()
                    )?;
                    log::debug!("Number of Bitbucket commits: {}", commits.len());
                    log::debug!("Number of Bitbucket pull requests: {}", pull_requests.len());
                    Ok((commits, pull_requests))
                })
        } else {
            Ok((vec![], vec![]))
        }
    }

    /// Returns the Azure DevOps metadata needed for the changelog.
    ///
    /// This function creates a multithread async runtime for handling the
    /// requests. The following are fetched from the Azure DevOps REST API:
    ///
    /// - Commits
    /// - Pull requests
    ///
    /// Each of these are paginated requests so they are being run in parallel
    /// for speedup.
    ///
    /// If no Azure DevOps related variable is used in the template then this
    /// function returns empty vectors.
    #[cfg(feature = "azure_devops")]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn get_azure_devops_metadata(
        &self,
        ref_name: Option<&str>,
    ) -> Result<crate::remote::RemoteMetadata> {
        use crate::remote::azure_devops;
        if self.config.remote.azure_devops.is_custom ||
            self.body_template
                .contains_variable(azure_devops::TEMPLATE_VARIABLES) ||
            self.footer_template
                .as_ref()
                .is_some_and(|v| v.contains_variable(azure_devops::TEMPLATE_VARIABLES))
        {
            let azure_devops_client =
                AzureDevOpsClient::try_from(self.config.remote.azure_devops.clone())?;
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    let (commits, pull_requests) = tokio::try_join!(
                        azure_devops_client.get_commits(ref_name),
                        azure_devops_client.get_pull_requests()
                    )?;
                    log::debug!("Number of Azure DevOps commits: {}", commits.len());
                    log::debug!(
                        "Number of Azure DevOps pull requests: {}",
                        pull_requests.len()
                    );
                    Ok((commits, pull_requests))
                })
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
    #[allow(unused_variables)]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn add_remote_data(&mut self, range: Option<&str>) -> Result<()> {
        log::debug!("Adding remote data");

        // Determine the ref at which to fetch remote commits, based on the commit
        // range
        let range_head = range.and_then(|r| r.split("..").last());
        let ref_name = match range_head {
            Some("HEAD") => None,
            other => other,
        };

        #[cfg(feature = "github")]
        let (github_commits, github_pull_requests) = if self.config.remote.github.is_set() {
            self.get_github_metadata(ref_name)
                .expect("Could not get github metadata")
        } else {
            (vec![], vec![])
        };
        #[cfg(feature = "gitlab")]
        let (gitlab_commits, gitlab_merge_request) = if self.config.remote.gitlab.is_set() {
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
        let (bitbucket_commits, bitbucket_pull_request) = if self.config.remote.bitbucket.is_set() {
            self.get_bitbucket_metadata(ref_name)
                .expect("Could not get bitbucket metadata")
        } else {
            (vec![], vec![])
        };
        #[cfg(feature = "azure_devops")]
        let (azure_devops_commits, azure_devops_pull_request) =
            if self.config.remote.azure_devops.is_set() {
                self.get_azure_devops_metadata(ref_name)
                    .expect("Could not get azure_devops metadata")
            } else {
                (vec![], vec![])
            };
        #[cfg(feature = "remote")]
        for release in &mut self.releases {
            #[cfg(feature = "github")]
            release.update_github_metadata(github_commits.clone(), github_pull_requests.clone())?;
            #[cfg(feature = "gitlab")]
            release.update_gitlab_metadata(gitlab_commits.clone(), gitlab_merge_request.clone())?;
            #[cfg(feature = "gitea")]
            release.update_gitea_metadata(gitea_commits.clone(), gitea_merge_request.clone())?;
            #[cfg(feature = "bitbucket")]
            release.update_bitbucket_metadata(
                bitbucket_commits.clone(),
                bitbucket_pull_request.clone(),
            )?;
            #[cfg(feature = "azure_devops")]
            release.update_azure_devops_metadata(
                azure_devops_commits.clone(),
                azure_devops_pull_request.clone(),
            )?;
        }
        Ok(())
    }

    /// Increments the version for the unreleased changes based on semver.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn bump_version(&mut self) -> Result<Option<String>> {
        if let Some(ref mut last_release) = self.releases.iter_mut().next() {
            if last_release.version.is_none() {
                let next_version =
                    last_release.calculate_next_version_with_config(&self.config.bump)?;
                log::debug!("Bumping the version to {next_version}");
                last_release.version = Some(next_version.clone());
                last_release.timestamp = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs()
                        .try_into()?,
                );
                return Ok(Some(next_version));
            }
        }
        Ok(None)
    }

    /// Generates the changelog and writes it to the given output.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn generate<W: Write + ?Sized>(&self, out: &mut W) -> Result<()> {
        log::debug!("Generating changelog");
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn prepend<W: Write + ?Sized>(&self, mut changelog: String, out: &mut W) -> Result<()> {
        log::debug!("Generating changelog and prepending");
        if let Some(header) = &self.config.changelog.header {
            changelog = changelog.replacen(header, "", 1);
        }
        self.generate(out)?;
        write!(out, "{changelog}")?;
        Ok(())
    }

    /// Prints the changelog context to the given output.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
        "commit.azure_devops",
    ];
    if template.contains_variable(&deprecated_vars) {
        log::warn!(
            "Variables {deprecated_vars:?} are deprecated and will be removed in the future. Use \
             `commit.remote` instead."
        );
    }
    Ok(template)
}

#[cfg(test)]
mod test {
    use std::str;

    use pretty_assertions::assert_eq;
    use regex::Regex;

    use super::*;
    use crate::commit::Signature;
    use crate::config::{
        Bump, ChangelogConfig, CommitParser, LinkParser, Remote, RemoteConfig, TextProcessor,
    };

    fn get_test_data() -> (Config, Vec<Release<'static>>) {
        let config = Config {
            changelog: ChangelogConfig {
                header: Some(String::from("# Changelog")),
                body: String::from(
                    r#"{% if version %}
				## Release [{{ version }}] - {{ timestamp | date(format="%Y-%m-%d") }} - ({{ repository }})
				{% if commit_id %}({{ commit_id }}){% endif %}{% else %}
				## Unreleased{% endif %}
				{% for group, commits in commits | group_by(attribute="group") %}
				### {{ group }}{% for group, commits in commits | group_by(attribute="scope") %}
				#### {{ group }}{% for commit in commits %}
				- {{ commit.message }}{% endfor %}
				{% endfor %}{% endfor %}
				### Commit Statistics

				- {{ statistics.commit_count }} commit(s) contributed to the release.
				- {{ statistics.commits_timespan | default(value=0) }} day(s) passed between the first and last commit.
				- {{ statistics.conventional_commit_count }} commit(s) parsed as conventional.
				- {{ statistics.links | length }} linked issue(s) detected in commits.
				{%- if statistics.links | length > 0 %}
					{%- for link in statistics.links %}
						{{ "  " }}- [{{ link.text }}]({{ link.href }}) (referenced {{ link.count }} time(s))
					{%- endfor %}
				{%- endif %}
				{%- if statistics.days_passed_since_last_release %}
					- {{ statistics.days_passed_since_last_release }} day(s) passed between releases.
				{%- endif %}
				"#,
                ),
                footer: Some(String::from(
                    r"-- total releases: {{ releases | length }} --",
                )),
                trim: true,
                postprocessors: vec![TextProcessor {
                    pattern: Regex::new("boring").expect("failed to compile regex"),
                    replace: Some(String::from("exciting")),
                    replace_command: None,
                }],
                render_always: false,
                output: None,
            },
            git: GitConfig {
                conventional_commits: true,
                require_conventional: false,
                filter_unconventional: false,
                split_commits: false,
                commit_preprocessors: vec![TextProcessor {
                    pattern: Regex::new("<preprocess>").expect("failed to compile regex"),
                    replace: Some(String::from("this commit is preprocessed")),
                    replace_command: None,
                }],
                commit_parsers: vec![
                    CommitParser {
                        sha: Some(String::from("tea")),
                        message: None,
                        body: None,
                        footer: None,
                        group: Some(String::from("I love tea")),
                        default_scope: None,
                        scope: None,
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: Some(String::from("coffee")),
                        message: None,
                        body: None,
                        footer: None,
                        group: None,
                        default_scope: None,
                        scope: None,
                        skip: Some(true),
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: Some(String::from("coffee2")),
                        message: None,
                        body: None,
                        footer: None,
                        group: None,
                        default_scope: None,
                        scope: None,
                        skip: Some(true),
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new(r".*merge.*").ok(),
                        body: None,
                        footer: None,
                        group: None,
                        default_scope: None,
                        scope: None,
                        skip: Some(true),
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new("feat*").ok(),
                        body: None,
                        footer: None,
                        group: Some(String::from("New features")),
                        default_scope: Some(String::from("other")),
                        scope: None,
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new("^fix*").ok(),
                        body: None,
                        footer: None,
                        group: Some(String::from("Bug Fixes")),
                        default_scope: None,
                        scope: None,
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new("doc:").ok(),
                        body: None,
                        footer: None,
                        group: Some(String::from("Documentation")),
                        default_scope: None,
                        scope: Some(String::from("documentation")),
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new("docs:").ok(),
                        body: None,
                        footer: None,
                        group: Some(String::from("Documentation")),
                        default_scope: None,
                        scope: Some(String::from("documentation")),
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new(r"match\((.*)\):.*").ok(),
                        body: None,
                        footer: None,
                        group: Some(String::from("Matched ($1)")),
                        default_scope: None,
                        scope: None,
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: None,
                        body: None,
                        footer: Regex::new("Footer:.*").ok(),
                        group: Some(String::from("Footer")),
                        default_scope: None,
                        scope: Some(String::from("footer")),
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                    CommitParser {
                        sha: None,
                        message: Regex::new(".*").ok(),
                        body: None,
                        footer: None,
                        group: Some(String::from("Other")),
                        default_scope: Some(String::from("other")),
                        scope: None,
                        skip: None,
                        field: None,
                        pattern: None,
                    },
                ],
                protect_breaking_commits: false,
                filter_commits: false,
                fail_on_unmatched_commit: false,
                tag_pattern: None,
                skip_tags: Regex::new("v3.*").ok(),
                ignore_tags: None,
                count_tags: None,
                use_branch_tags: false,
                topo_order: false,
                topo_order_commits: true,
                sort_commits: String::from("oldest"),
                link_parsers: vec![LinkParser {
                    pattern: Regex::new("#(\\d+)").expect("issue reference regex should be valid"),
                    href: String::from("https://github.com/$1"),
                    text: None,
                }],
                limit_commits: None,
                recurse_submodules: None,
                include_paths: Vec::new(),
                exclude_paths: Vec::new(),
            },
            remote: RemoteConfig {
                offline: false,
                github: Remote {
                    owner: String::from("coolguy"),
                    repo: String::from("awesome"),
                    token: None,
                    is_custom: false,
                    api_url: None,
                    native_tls: None,
                },
                gitlab: Remote {
                    owner: String::from("coolguy"),
                    repo: String::from("awesome"),
                    token: None,
                    is_custom: false,
                    api_url: None,
                    native_tls: None,
                },
                gitea: Remote {
                    owner: String::from("coolguy"),
                    repo: String::from("awesome"),
                    token: None,
                    is_custom: false,
                    api_url: None,
                    native_tls: None,
                },
                bitbucket: Remote {
                    owner: String::from("coolguy"),
                    repo: String::from("awesome"),
                    token: None,
                    is_custom: false,
                    api_url: None,
                    native_tls: None,
                },
                azure_devops: Remote {
                    owner: String::from("coolguy"),
                    repo: String::from("awesome"),
                    token: None,
                    is_custom: false,
                    api_url: None,
                    native_tls: None,
                },
            },
            bump: Bump::default(),
        };
        let test_release = Release {
            version: Some(String::from("v1.0.0")),
            message: None,
            extra: None,
            commits: vec![
                Commit {
                    id: String::from("coffee"),
                    message: String::from("revert(app): skip this commit"),
                    committer: Signature {
                        timestamp: 48_704_000,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("tea"),
                    message: String::from("feat(app): damn right"),
                    committer: Signature {
                        timestamp: 48_790_400,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("0bc123"),
                    message: String::from("feat(app): add cool features"),
                    committer: Signature {
                        timestamp: 48_876_800,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("000000"),
                    message: String::from("support unconventional commits"),
                    committer: Signature {
                        timestamp: 48_963_200,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("0bc123"),
                    message: String::from("feat: support unscoped commits"),
                    committer: Signature {
                        timestamp: 49_049_600,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("0werty"),
                    message: String::from("style(ui): make good stuff"),
                    committer: Signature {
                        timestamp: 49_136_000,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("0w3rty"),
                    message: String::from("fix(ui): fix more stuff"),
                    committer: Signature {
                        timestamp: 49_222_400,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("qw3rty"),
                    message: String::from("doc: update docs"),
                    committer: Signature {
                        timestamp: 49_308_800,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("0bc123"),
                    message: String::from("docs: add some documentation"),
                    committer: Signature {
                        timestamp: 49_395_200,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("0jkl12"),
                    message: String::from("chore(app): do nothing"),
                    committer: Signature {
                        timestamp: 49_481_600,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("qwerty"),
                    message: String::from("chore: <preprocess>"),
                    committer: Signature {
                        timestamp: 49_568_000,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("qwertz"),
                    message: String::from("feat!: support breaking commits"),
                    committer: Signature {
                        timestamp: 49_654_400,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("qwert0"),
                    message: String::from("match(group): support regex-replace for groups"),
                    committer: Signature {
                        timestamp: 49_740_800,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("coffee"),
                    message: String::from("revert(app): skip this commit"),
                    committer: Signature {
                        timestamp: 49_827_200,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Commit {
                    id: String::from("footer"),
                    message: String::from("misc: use footer\n\nFooter: footer text"),
                    committer: Signature {
                        timestamp: 49_913_600,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            commit_range: None,
            commit_id: Some(String::from("0bc123")),
            timestamp: Some(50_000_000),
            previous: None,
            repository: Some(String::from("/root/repo")),
            submodule_commits: HashMap::from([(String::from("submodule_one"), vec![
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
                    String::from("feat!: submodule_one support breaking commits"),
                ),
                Commit::new(
                    String::from("subqwert0"),
                    String::from("match(group): submodule_one support regex-replace for groups"),
                ),
            ])]),
            statistics: None,
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
            #[cfg(feature = "azure_devops")]
            azure_devops: crate::remote::RemoteReleaseMetadata {
                contributors: vec![],
            },
        };
        let releases = vec![
            test_release.clone(),
            Release {
                version: Some(String::from("v3.0.0")),
                commits: vec![Commit {
                    id: String::from("n0thin"),
                    message: String::from("feat(xyz): skip commit"),
                    committer: Signature {
                        timestamp: 49_913_600,
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Release::default()
            },
            Release {
                version: None,
                message: None,
                extra: None,
                commits: vec![
                    Commit {
                        id: String::from("abc123"),
                        message: String::from("feat(app): add xyz"),
                        committer: Signature {
                            timestamp: 49_395_200,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Commit {
                        id: String::from("abc124"),
                        message: String::from("docs(app): document zyx"),
                        committer: Signature {
                            timestamp: 49_481_600,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Commit {
                        id: String::from("def789"),
                        message: String::from("merge #4"),
                        committer: Signature {
                            timestamp: 49_568_000,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Commit {
                        id: String::from("dev063"),
                        message: String::from("feat(app)!: merge #5"),
                        committer: Signature {
                            timestamp: 49_654_400,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Commit {
                        id: String::from("qwerty"),
                        message: String::from("fix(app): fix abc"),
                        committer: Signature {
                            timestamp: 49_740_800,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Commit {
                        id: String::from("hjkl12"),
                        message: String::from("chore(ui): do boring stuff"),
                        committer: Signature {
                            timestamp: 49_827_200,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Commit {
                        id: String::from("coffee2"),
                        message: String::from("revert(app): skip this commit"),
                        committer: Signature {
                            timestamp: 49_913_600,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                commit_range: None,
                commit_id: None,
                timestamp: Some(1000),
                previous: Some(Box::new(test_release)),
                repository: Some(String::from("/root/repo")),
                submodule_commits: HashMap::from([
                    (String::from("submodule_one"), vec![
                        Commit::new(String::from("def349"), String::from("sub_one merge #4")),
                        Commit::new(String::from("da8912"), String::from("sub_one merge #5")),
                    ]),
                    (String::from("submodule_two"), vec![Commit::new(
                        String::from("ab76ef"),
                        String::from("sub_two bump"),
                    )]),
                ]),
                statistics: None,
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
                #[cfg(feature = "azure_devops")]
                azure_devops: crate::remote::RemoteReleaseMetadata {
                    contributors: vec![],
                },
            },
        ];
        (config, releases)
    }

    #[test]
    fn changelog_generator() -> Result<()> {
        let (config, releases) = get_test_data();

        let mut changelog = Changelog::new(releases, config, None)?;
        changelog.bump_version()?;
        changelog.releases[0].timestamp = Some(0);
        let mut out = Vec::new();
        changelog.generate(&mut out)?;
        assert_eq!(
            String::from(
                r"# Changelog

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

			### Commit Statistics

			- 4 commit(s) contributed to the release.
			- 5 day(s) passed between the first and last commit.
			- 4 commit(s) parsed as conventional.
			- 0 linked issue(s) detected in commits.
			- -578 day(s) passed between releases.

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

			### Commit Statistics

			- 13 commit(s) contributed to the release.
			- 13 day(s) passed between the first and last commit.
			- 12 commit(s) parsed as conventional.
			- 0 linked issue(s) detected in commits.
			-- total releases: 2 --
			"
            )
            .replace("			", ""),
            str::from_utf8(&out).unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn changelog_generator_render_always() -> Result<()> {
        let (mut config, mut releases) = get_test_data();
        config.changelog.render_always = true;

        releases[0].commits = Vec::new();
        releases[2].commits = Vec::new();
        releases[2].previous = Some(Box::new(releases[0].clone()));
        let changelog = Changelog::new(releases, config, None)?;
        let mut out = Vec::new();
        changelog.generate(&mut out)?;
        assert_eq!(
            String::from(
                r"# Changelog

			## Unreleased

			### Commit Statistics

			- 0 commit(s) contributed to the release.
			- 0 day(s) passed between the first and last commit.
			- 0 commit(s) parsed as conventional.
			- 0 linked issue(s) detected in commits.
			- -578 day(s) passed between releases.
			-- total releases: 1 --
			"
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

        releases[0].commits.push(Commit {
            id: String::from("0bc123"),
            message: String::from(
                "feat(app): add some more cool features
feat(app): even more features
feat(app): feature #3
",
            ),
            committer: Signature {
                timestamp: 49_827_200,
                ..Default::default()
            },
            ..Default::default()
        });
        releases[0].commits.push(Commit {
            id: String::from("003934"),
            message: String::from(
                "feat: add awesome stuff
fix(backend): fix awesome stuff
style: make awesome stuff look better
",
            ),
            committer: Signature {
                timestamp: 49_740_800,
                ..Default::default()
            },
            ..Default::default()
        });
        releases[2].commits.push(Commit {
            id: String::from("123abc"),
            message: String::from(
                "chore(deps): bump some deps

chore(deps): bump some more deps
chore(deps): fix broken deps
",
            ),
            committer: Signature {
                timestamp: 49_308_800,
                ..Default::default()
            },
            ..Default::default()
        });
        let changelog = Changelog::new(releases, config, None)?;
        let mut out = Vec::new();
        changelog.generate(&mut out)?;
        assert_eq!(
            String::from(
                r"# Changelog

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

			### Commit Statistics

			- 8 commit(s) contributed to the release.
			- 6 day(s) passed between the first and last commit.
			- 8 commit(s) parsed as conventional.
			- 1 linked issue(s) detected in commits.
			  - [#5](https://github.com/5) (referenced 1 time(s))
			- -578 day(s) passed between releases.

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

			### Commit Statistics

			- 18 commit(s) contributed to the release.
			- 12 day(s) passed between the first and last commit.
			- 17 commit(s) parsed as conventional.
			- 1 linked issue(s) detected in commits.
			  - [#3](https://github.com/3) (referenced 1 time(s))
			-- total releases: 2 --
			"
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
        let mut changelog = Changelog::new(releases, config, None)?;
        changelog.add_context("custom_field", "Hello")?;
        let mut out = Vec::new();
        changelog.generate(&mut out)?;
        expect_test::expect![[r"
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
"]]
        .assert_eq(str::from_utf8(&out).unwrap_or_default());
        Ok(())
    }
}
