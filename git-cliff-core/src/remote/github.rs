use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};
use std::env;

use super::*;

/// GitHub REST API url.
const GITHUB_API_URL: &str = "https://api.github.com";

/// Environment variable for overriding the GitHub REST API url.
const GITHUB_API_URL_ENV: &str = "GITHUB_API_URL";

/// Log message to show while fetching data from GitHub.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitHub...";

/// Log message to show when done fetching from GitHub.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitHub data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["github", "commit.github"];

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommit {
	/// SHA.
	pub sha:    String,
	/// Author of the commit.
	pub author: Option<GitHubCommitAuthor>,
}

impl RemoteCommit for GitHubCommit {
	fn id(&self) -> String {
		self.sha.clone()
	}

	fn username(&self) -> Option<String> {
		self.author.clone().and_then(|v| v.login)
	}
}

impl RemoteEntry for GitHubCommit {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: i32) -> String {
		format!(
			"{}/repos/{}/{}/commits?per_page={MAX_PAGE_SIZE}&page={page}",
			api_url, remote.owner, remote.repo
		)
	}
	fn buffer_size() -> usize {
		10
	}

	fn early_exit(&self) -> bool {
		false
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
	pub number:           i64,
	/// Pull request title.
	pub title:            Option<String>,
	/// SHA of the merge commit.
	pub merge_commit_sha: Option<String>,
	/// Labels of the pull request.
	pub labels:           Vec<PullRequestLabel>,
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

impl RemoteEntry for GitHubPullRequest {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: i32) -> String {
		format!(
			"{}/repos/{}/{}/pulls?per_page={MAX_PAGE_SIZE}&page={page}&state=closed",
			api_url, remote.owner, remote.repo
		)
	}

	fn buffer_size() -> usize {
		5
	}

	fn early_exit(&self) -> bool {
		false
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
			client: create_remote_client(&remote, "application/vnd.github+json")?,
			remote,
		})
	}
}

impl RemoteClient for GitHubClient {
	fn api_url() -> String {
		env::var(GITHUB_API_URL_ENV)
			.ok()
			.unwrap_or_else(|| GITHUB_API_URL.to_string())
	}

	fn remote(&self) -> Remote {
		self.remote.clone()
	}

	fn client(&self) -> ClientWithMiddleware {
		self.client.clone()
	}
}

impl GitHubClient {
	/// Fetches the GitHub API and returns the commits.
	pub async fn get_commits(&self) -> Result<Vec<Box<dyn RemoteCommit>>> {
		Ok(self
			.fetch::<GitHubCommit>(0)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemoteCommit>)
			.collect())
	}

	/// Fetches the GitHub API and returns the pull requests.
	pub async fn get_pull_requests(
		&self,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		Ok(self
			.fetch::<GitHubPullRequest>(0)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
			.collect())
	}
}
