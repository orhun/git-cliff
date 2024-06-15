use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};
use std::env;

use super::*;

/// Gitea API url.
const GITEA_API_URL: &str = "https://codeberg.org";

/// Environment variable for overriding the Gitea REST API url.
const GITEA_API_URL_ENV: &str = "GITEA_API_URL";

/// Log message to show while fetching data from Gitea.
pub const START_FETCHING_MSG: &str = "Retrieving data from Gitea...";

/// Log message to show when done fetching from Gitea.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching Gitea data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["gitea", "commit.gitea"];

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiteaCommit {
	/// SHA.
	pub sha:    String,
	/// Author of the commit.
	pub author: Option<GiteaCommitAuthor>,
}

impl RemoteCommit for GiteaCommit {
	fn id(&self) -> String {
		self.sha.clone()
	}

	fn username(&self) -> Option<String> {
		self.author.clone().and_then(|v| v.login)
	}
}

impl RemoteEntry for GiteaCommit {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: i32) -> String {
		format!(
			"{}/api/v1/repos/{}/{}/commits?limit={MAX_PAGE_SIZE}&page={page}",
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
pub struct GiteaCommitAuthor {
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
pub struct GiteaPullRequest {
	/// Pull request number.
	pub number:           i64,
	/// Pull request title.
	pub title:            Option<String>,
	/// SHA of the merge commit.
	pub merge_commit_sha: Option<String>,
	/// Labels of the pull request.
	pub labels:           Vec<PullRequestLabel>,
}

impl RemotePullRequest for GiteaPullRequest {
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

impl RemoteEntry for GiteaPullRequest {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: i32) -> String {
		format!(
			"{}/api/v1/repos/{}/{}/pulls?limit={MAX_PAGE_SIZE}&page={page}&\
			 state=closed",
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

/// HTTP client for handling Gitea REST API requests.
#[derive(Debug, Clone)]
pub struct GiteaClient {
	/// Remote.
	remote: Remote,
	/// HTTP client.
	client: ClientWithMiddleware,
}

/// Constructs a Gitea client from the remote configuration.
impl TryFrom<Remote> for GiteaClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		Ok(Self {
			client: create_remote_client(&remote, "application/json")?,
			remote,
		})
	}
}

impl RemoteClient for GiteaClient {
	fn api_url() -> String {
		env::var(GITEA_API_URL_ENV)
			.ok()
			.unwrap_or_else(|| GITEA_API_URL.to_string())
	}

	fn remote(&self) -> Remote {
		self.remote.clone()
	}

	fn client(&self) -> ClientWithMiddleware {
		self.client.clone()
	}
}

impl GiteaClient {
	/// Fetches the Gitea API and returns the commits.
	pub async fn get_commits(&self) -> Result<Vec<Box<dyn RemoteCommit>>> {
		Ok(self
			.fetch::<GiteaCommit>(0)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemoteCommit>)
			.collect())
	}

	/// Fetches the Gitea API and returns the pull requests.
	pub async fn get_pull_requests(
		&self,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		Ok(self
			.fetch::<GiteaPullRequest>(0)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
			.collect())
	}
}
