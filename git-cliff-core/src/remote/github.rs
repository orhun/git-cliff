use std::fmt::Write;

use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::{Debug, MAX_PAGE_SIZE, RemoteClient, RemoteCommit, RemotePullRequest};
use crate::config::Remote;
use crate::error::{Error, Result};

/// Log message to show while fetching data from GitHub.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitHub...";

/// Log message to show when done fetching from GitHub.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitHub data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["github", "commit.github", "commit.remote"];

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommit {
    /// SHA.
    pub sha: String,
    /// Author of the commit.
    pub author: Option<GitHubCommitAuthor>,
    /// Details of the commit
    pub commit: Option<GitHubCommitDetails>,
}

/// Representation of subset of commit details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommitDetails {
    /// Author of the commit
    pub author: GitHubCommitDetailsAuthor,
}

/// Representation of subset of commit author details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommitDetailsAuthor {
    /// Date of the commit
    pub date: String,
}

impl RemoteCommit for GitHubCommit {
    fn id(&self) -> String {
        self.sha.clone()
    }

    fn username(&self) -> Option<String> {
        self.author.clone().and_then(|v| v.login)
    }

    fn timestamp(&self) -> Option<i64> {
        self.commit
            .clone()
            .map(|f| self.convert_to_unix_timestamp(f.author.date.clone().as_str()))
    }
}

/// Author of the commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommitAuthor {
    /// Username.
    pub login: Option<String>,
}

/// Label of the pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestLabel {
    /// Name of the label.
    pub name: String,
}

/// Representation of a single pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequest {
    /// Pull request number.
    pub number: i64,
    /// Pull request title.
    pub title: Option<String>,
    /// SHA of the merge commit.
    pub merge_commit_sha: Option<String>,
    /// Labels of the pull request.
    pub labels: Vec<PullRequestLabel>,
}

impl RemotePullRequest for GitHubPullRequest {
    fn number(&self) -> i64 {
        self.number
    }

    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn labels(&self) -> Vec<String> {
        self.labels.iter().map(|v| v.name.clone()).collect()
    }

    fn merge_commit(&self) -> Option<String> {
        self.merge_commit_sha.clone()
    }
}

/// HTTP client for handling GitHub REST API requests.
#[derive(Debug, Clone)]
pub struct GitHubClient {
    /// Remote.
    remote: Remote,
    /// HTTP client.
    client: ClientWithMiddleware,
}

/// Constructs a GitHub client from the remote configuration.
impl TryFrom<Remote> for GitHubClient {
    type Error = Error;
    fn try_from(remote: Remote) -> Result<Self> {
        Ok(Self {
            client: remote.create_client("application/vnd.github+json")?,
            remote,
        })
    }
}

impl RemoteClient for GitHubClient {
    const API_URL: &'static str = "https://api.github.com";
    const API_URL_ENV: &'static str = "GITHUB_API_URL";

    fn remote(&self) -> Remote {
        self.remote.clone()
    }

    fn client(&self) -> ClientWithMiddleware {
        self.client.clone()
    }
}

impl GitHubClient {
    /// Constructs the URL for GitHub commits API.
    fn commits_url(api_url: &str, remote: &Remote, ref_name: Option<&str>, page: i32) -> String {
        let mut url = format!(
            "{}/repos/{}/{}/commits?per_page={MAX_PAGE_SIZE}&page={page}",
            api_url, remote.owner, remote.repo
        );

        if let Some(ref_name) = ref_name {
            write!(url, "&sha={}", urlencoding::encode(ref_name))
                .expect("Writing ref name query should never fail");
        }

        url
    }

    /// Constructs the URL for GitHub pull requests API.
    fn pull_requests_url(api_url: &str, remote: &Remote, page: i32) -> String {
        format!(
            "{}/repos/{}/{}/pulls?per_page={MAX_PAGE_SIZE}&page={page}&state=closed",
            api_url, remote.owner, remote.repo
        )
    }

    /// Fetches the complete list of commits.
    /// This is inefficient for large repositories; consider using
    /// `get_commit_stream` instead.
    pub async fn get_commits(&self, ref_name: Option<&str>) -> Result<Vec<Box<dyn RemoteCommit>>> {
        use futures::TryStreamExt;
        self.get_commit_stream(ref_name).try_collect().await
    }

    /// Fetches the complete list of pull requests.
    /// This is inefficient for large repositories; consider using
    /// `get_pull_request_stream` instead.
    pub async fn get_pull_requests(&self) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        use futures::TryStreamExt;
        self.get_pull_request_stream().try_collect().await
    }

    fn get_commit_stream(
        &self,
        ref_name: Option<&str>,
    ) -> impl Stream<Item = Result<Box<dyn RemoteCommit>>> + '_ {
        let ref_name = ref_name.map(ToString::to_string);
        async_stream! {
            let page_stream = stream::iter(0..)
                .map(|page|
                    {
                    let ref_name = ref_name.clone();
                    async move {
                        let url = Self::commits_url(&self.api_url(), &self.remote(), ref_name.as_deref(), page);
                        self.get_json::<Vec<GitHubCommit>>(&url).await
                    }})
                .buffered(10);

            let mut page_stream = Box::pin(page_stream);

            while let Some(page_result) = page_stream.next().await {
                match page_result {
                    Ok(commits) => {
                        if commits.is_empty() {
                            break;
                        }

                        for commit in commits {
                            yield Ok(Box::new(commit) as Box<dyn RemoteCommit>);
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                        break;
                    }
                }
            }
        }
    }

    fn get_pull_request_stream(
        &self,
    ) -> impl Stream<Item = Result<Box<dyn RemotePullRequest>>> + '_ {
        async_stream! {
            let page_stream = stream::iter(0..)
                .map(|page| async move {
                    let url = Self::pull_requests_url(&self.api_url(), &self.remote(), page);
                    self.get_json::<Vec<GitHubPullRequest>>(&url).await
                })
                .buffered(5);

            let mut page_stream = Box::pin(page_stream);

            while let Some(page_result) = page_stream.next().await {
                match page_result {
                    Ok(prs) => {
                        if prs.is_empty() {
                            break;
                        }

                        for pr in prs {
                            yield Ok(Box::new(pr) as Box<dyn RemotePullRequest>);
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::remote::RemoteCommit;

    #[test]
    fn timestamp() {
        let remote_commit = GitHubCommit {
            sha: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
            author: Some(GitHubCommitAuthor {
                login: Some(String::from("orhun")),
            }),
            commit: Some(GitHubCommitDetails {
                author: GitHubCommitDetailsAuthor {
                    date: String::from("2021-07-18T15:14:39+03:00"),
                },
            }),
        };

        assert_eq!(Some(1_626_610_479), remote_commit.timestamp());
    }
}
