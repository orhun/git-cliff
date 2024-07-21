use super::{
	RemoteClient,
	MAX_PAGE_PRS,
};
use crate::config::Remote;
use crate::error::Result;
use crate::remote::{
	RemoteCommit,
	RemoteEntry,
	RemotePullRequest,
	MAX_PAGE_SIZE,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};

/// Bitbucket REST API url.
const API_URL: &str = "https://api.bitbucket.org/2.0/repositories";

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
	/// SHA.
	pub hash:   String,
	/// Author of the commit.
	pub author: Option<CommitAuthor>,
}

impl RemoteCommit for Commit {
	fn id(&self) -> String {
		self.hash.clone()
	}

	fn username(&self) -> Option<String> {
		self.author.clone().and_then(|v| v.login)
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-commits-get>
impl RemoteEntry for Pagination<Commit> {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: usize) -> String {
		let commit_page = page + 1;
		format!(
			"{}/{}/{}/commits?pagelen={MAX_PAGE_SIZE}&page={commit_page}",
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
pub struct Pagination<T> {
	/// Total number of objects in the response.
	pub size:     Option<i64>,
	/// Page number of the current results.
	pub page:     Option<i64>,
	/// Current number of objects on the existing page.  Globally, the minimum
	/// length is 10 and the maximum is 100.
	pub pagelen:  Option<i64>,
	/// Link to the next page if it exists.
	pub next:     Option<String>,
	/// Link to the previous page if it exists.
	pub previous: Option<String>,
	/// List of Objects.
	pub values:   Vec<T>,
}

/// Author of the commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitAuthor {
	/// Username.
	#[serde(rename = "raw")]
	pub login: Option<String>,
}

/// Representation of a single pull request's merge commit
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMergeCommit {
	/// SHA of the merge commit.
	pub hash: String,
}

/// Representation of a single pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
	/// Pull request number.
	pub id:               u64,
	/// Pull request title.
	pub title:            Option<String>,
	/// Pull request merge commit.
	pub merge_commit_sha: PullRequestMergeCommit,
	/// Author of the pull request.
	pub author:           CommitAuthor,
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
		Some(&self.merge_commit_sha.hash)
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-pullrequests/#api-repositories-workspace-repo-slug-pullrequests-get>
impl RemoteEntry for Pagination<PullRequest> {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: usize) -> String {
		let pr_page = page + 1;
		format!(
			"{}/{}/{}/pullrequests?&pagelen={MAX_PAGE_PRS}&page={pr_page}&\
			 state=MERGED",
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
	remote: Remote,
	/// HTTP client.
	client: ClientWithMiddleware,
}

impl From<(Remote, ClientWithMiddleware)> for Client {
	fn from((remote, client): (Remote, ClientWithMiddleware)) -> Self {
		Self { remote, client }
	}
}

impl RemoteClient for Client {
	fn api_url(&self) -> String {
		API_URL.to_string()
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
