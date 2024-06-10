use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};
use url::Url;

use super::*;

/// Gitea REST API url.
const GITEA_API_URL_CFG: ApiUrlCfg = ApiUrlCfg {
	api_url:        "https://codeberg.org/api/v1",
	env_var:        "GITEA_API_URL",
	api_path:       "/api/v1",
	default_domain: "",
};

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiteaCommit {
	/// SHA.
	pub sha:    String,
	/// Author of the commit.
	pub author: Option<GiteaCommitAuthor>,
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

impl RemoteEntry for GiteaCommit {
	fn url(_project_id: i64, api_url: &Url, remote: &Remote, page: i32) -> String {
		format!(
			"{}/repos/{}/{}/commits?limit={MAX_PAGE_SIZE}&page={page}",
			api_url, remote.owner, remote.repo
		)
	}

	fn buffer_size() -> usize {
		10
	}
}

impl RemoteEntry for GiteaPullRequest {
	fn url(_project_id: i64, api_url: &Url, remote: &Remote, page: i32) -> String {
		format!(
			"{}/repos/{}/{}/pulls?limit={MAX_PAGE_SIZE}&page={page}&state=closed",
			api_url, remote.owner, remote.repo
		)
	}

	fn buffer_size() -> usize {
		5
	}
}

impl From<GiteaCommit> for RemoteCommit {
	fn from(value: GiteaCommit) -> Self {
		Self {
			id:       value.sha,
			username: value.author.and_then(|a| a.login),
		}
	}
}

impl From<GiteaPullRequest> for RemotePullRequest {
	fn from(value: GiteaPullRequest) -> Self {
		Self {
			number:       value.number,
			title:        value.title,
			labels:       value.labels.into_iter().map(|l| l.name).collect(),
			merge_commit: value.merge_commit_sha,
		}
	}
}

/// HTTP client for handling Gitea REST API requests.
#[derive(Debug, Clone)]
pub struct GiteaClient {
	/// Remote.
	remote:  Remote,
	/// Gitea API Url
	api_url: Url,
	/// HTTP client.
	client:  ClientWithMiddleware,
}

/// Constructs a GitHub client from the remote configuration.
impl TryFrom<Remote> for GiteaClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		Ok(Self {
			client: create_remote_client(&remote, "application/json")?,
			api_url: GITEA_API_URL_CFG.get_api_url(&remote)?,
			remote,
		})
	}
}

impl RemoteClientInternal for GiteaClient {
	fn api_url(&self) -> &Url {
		&self.api_url
	}

	fn remote(&self) -> Remote {
		self.remote.clone()
	}

	fn client(&self) -> ClientWithMiddleware {
		self.client.clone()
	}
}

#[async_trait]
impl RemoteClient for GiteaClient {
	async fn get_commits(&self) -> Result<Vec<RemoteCommit>> {
		Ok(self
			.fetch::<GiteaCommit>(0)
			.await?
			.into_iter()
			.map(RemoteCommit::from)
			.collect())
	}

	async fn get_pull_requests(&self) -> Result<Vec<RemotePullRequest>> {
		Ok(self
			.fetch::<GiteaPullRequest>(0)
			.await?
			.into_iter()
			.map(RemotePullRequest::from)
			.collect())
	}
}
