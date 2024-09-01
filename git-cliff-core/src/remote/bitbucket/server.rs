use super::RemoteClient;
use crate::config::Remote;
use crate::error::Result;
use crate::remote::{
	RemoteCommit,
	RemoteEntry,
	RemotePullRequest,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};

/// Maximum number of entries to fetch for commits.
const MAX_PAGE_COMMITS: usize = 25;

/// Maximum number of entries to fetch for pull requests.
const MAX_PAGE_PRS: usize = 50;

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
	/// SHA.
	pub id:     String,
	/// Author of the commit.
	pub author: Option<CommitAuthor>,
}

impl RemoteCommit for Commit {
	fn id(&self) -> String {
		self.id.clone()
	}

	fn username(&self) -> Option<String> {
		self.author.clone().and_then(|v| v.login)
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-commits-get>
impl RemoteEntry for Pagination<Commit> {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: usize) -> String {
		let start = page * MAX_PAGE_COMMITS;
		format!(
			"{}/rest/api/1.0/projects/{}/repos/{}/commits?limit={MAX_PAGE_COMMITS}&\
			 start={start}",
			api_url, remote.owner, remote.repo
		)
	}

	fn buffer_size() -> usize {
		10
	}

	fn early_exit(&self) -> bool {
		self.values.is_empty()
	}
}

/// Pagination header.
///
/// <https://developer.atlassian.com/cloud/bitbucket/rest/intro/#pagination>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T> {
	/// Current number of objects on the existing page.
	pub size:            u32,
	/// Maximum amount of items per page.
	pub limit:           u32,
	/// Whether this is the last page.
	pub is_last_page:    bool,
	/// The start offset from the whole list.
	pub start:           u32,
	/// Start value to go to the next page.
	pub next_page_start: u32,
	/// List of Objects.
	pub values:          Vec<T>,
}

/// Author of the commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitAuthor {
	/// Username.
	#[serde(rename = "name")]
	pub login: Option<String>,
}

/// Representation of a single pull request's merge commit
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestMergeCommit {
	/// SHA of the merge commit.
	pub latest_commit: String,
}

/// Representation of a single pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
	/// Pull request number.
	pub id:       u64,
	/// Pull request title.
	pub title:    Option<String>,
	/// Pull request merge commit.
	pub from_ref: PullRequestMergeCommit,
	/// Author of the pull request.
	pub author:   CommitAuthor,
}

impl RemotePullRequest for PullRequest {
	fn number(&self) -> u64 {
		self.id
	}

	fn title(&self) -> Option<&str> {
		self.title.as_deref()
	}

	fn labels(&self) -> Vec<String> {
		vec![]
	}

	fn merge_commit(&self) -> Option<&str> {
		Some(&self.from_ref.latest_commit)
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-pullrequests/#api-repositories-workspace-repo-slug-pullrequests-get>
impl RemoteEntry for Pagination<PullRequest> {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: usize) -> String {
		let start = page * MAX_PAGE_PRS;
		format!(
			"{}/rest/api/1.0/projects/{}/repos/{}/pull-requests?\
			 limit={MAX_PAGE_PRS}&start={start}&state=MERGED",
			api_url, remote.owner, remote.repo
		)
	}

	fn buffer_size() -> usize {
		5
	}

	fn early_exit(&self) -> bool {
		self.values.is_empty()
	}
}

/// HTTP client for handling REST API requests.
#[derive(Debug, Clone)]
pub struct Client {
	/// Remote.
	remote:  Remote,
	/// HTTP client.
	client:  ClientWithMiddleware,
	api_url: String,
}

impl From<(Remote, ClientWithMiddleware, String)> for Client {
	fn from(
		(remote, client, api_url): (Remote, ClientWithMiddleware, String),
	) -> Self {
		Self {
			remote,
			client,
			api_url,
		}
	}
}

impl RemoteClient for Client {
	fn api_url(&self) -> String {
		self.api_url.clone()
	}

	fn remote(&self) -> Remote {
		self.remote.clone()
	}

	fn client(&self) -> ClientWithMiddleware {
		self.client.clone()
	}
}

impl Client {
	/// Fetches from the API and returns the commits.
	pub async fn get_commits(&self) -> Result<Vec<Box<dyn RemoteCommit>>> {
		Ok(self
			.fetch_with_early_exit::<Pagination<Commit>>(0)
			.await?
			.into_iter()
			.flat_map(|v| v.values)
			.map(|v| Box::new(v) as Box<dyn RemoteCommit>)
			.collect())
	}

	/// Fetches from the API and returns the pull requests.
	pub async fn get_pull_requests(
		&self,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		Ok(self
			.fetch_with_early_exit::<Pagination<PullRequest>>(0)
			.await?
			.into_iter()
			.flat_map(|v| v.values)
			.map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
			.collect())
	}
}
