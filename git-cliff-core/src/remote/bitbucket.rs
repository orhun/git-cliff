use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};
use std::env;

use super::*;

/// Bitbucket REST API url.
const BITBUCKET_API_URL: &str = "https://api.bitbucket.org/2.0/repositories";

/// Environment variable for overriding the Bitbucket REST API url.
const BITBUCKET_API_URL_ENV: &str = "BITBUCKET_API_URL";

/// Log message to show while fetching data from Bitbucket.
pub const START_FETCHING_MSG: &str = "Retrieving data from Bitbucket...";

/// Log message to show when done fetching from Bitbucket.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching Bitbucket data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["bitbucket", "commit.bitbucket"];

/// Maximum number of entries to fetch for bitbucket pull requests.
pub(crate) const BITBUCKET_MAX_PAGE_PRS: usize = 50;

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketCommit {
	/// SHA.
	pub hash:   String,
	/// Author of the commit.
	pub author: Option<BitbucketCommitAuthor>,
}

impl RemoteCommit for BitbucketCommit {
	fn id(&self) -> String {
		self.hash.clone()
	}

	fn username(&self) -> Option<String> {
		self.author.clone().and_then(|v| v.login)
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-commits-get>
impl RemoteEntry for BitbucketPagination<BitbucketCommit> {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: i32) -> String {
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

/// Bitbucket Pagination Header
///
/// <https://developer.atlassian.com/cloud/bitbucket/rest/intro/#pagination>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketPagination<T> {
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
pub struct BitbucketCommitAuthor {
	/// Username.
	#[serde(rename = "raw")]
	pub login: Option<String>,
}

/// Label of the pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestLabel {
	/// Name of the label.
	pub name: String,
}

/// Representation of a single pull request's merge commit
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketPullRequestMergeCommit {
	/// SHA of the merge commit.
	pub hash: String,
}

/// Representation of a single pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketPullRequest {
	/// Pull request number.
	pub id:               i64,
	/// Pull request title.
	pub title:            Option<String>,
	/// Bitbucket Pull Request Merge Commit
	pub merge_commit_sha: BitbucketPullRequestMergeCommit,
	/// Author of Pull Request
	pub author:           BitbucketCommitAuthor,
}

impl RemotePullRequest for BitbucketPullRequest {
	fn number(&self) -> i64 {
		self.id
	}

	fn title(&self) -> Option<String> {
		self.title.clone()
	}

	fn labels(&self) -> Vec<String> {
		vec![]
	}

	fn merge_commit(&self) -> Option<String> {
		Some(self.merge_commit_sha.hash.clone())
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-pullrequests/#api-repositories-workspace-repo-slug-pullrequests-get>
impl RemoteEntry for BitbucketPagination<BitbucketPullRequest> {
	fn url(_id: i64, api_url: &str, remote: &Remote, page: i32) -> String {
		let pr_page = page + 1;
		format!(
			"{}/{}/{}/pullrequests?&pagelen={BITBUCKET_MAX_PAGE_PRS}&\
			 page={pr_page}&state=MERGED",
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

/// HTTP client for handling Bitbucket REST API requests.
#[derive(Debug, Clone)]
pub struct BitbucketClient {
	/// Remote.
	remote: Remote,
	/// HTTP client.
	client: ClientWithMiddleware,
}

/// Constructs a Bitbucket client from the remote configuration.
impl TryFrom<Remote> for BitbucketClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		Ok(Self {
			client: create_remote_client(&remote, "application/json")?,
			remote,
		})
	}
}

impl RemoteClient for BitbucketClient {
	fn api_url() -> String {
		env::var(BITBUCKET_API_URL_ENV)
			.ok()
			.unwrap_or_else(|| BITBUCKET_API_URL.to_string())
	}

	fn remote(&self) -> Remote {
		self.remote.clone()
	}

	fn client(&self) -> ClientWithMiddleware {
		self.client.clone()
	}
}

impl BitbucketClient {
	/// Fetches the Bitbucket API and returns the commits.
	pub async fn get_commits(&self) -> Result<Vec<Box<dyn RemoteCommit>>> {
		Ok(self
			.fetch_with_early_exit::<BitbucketPagination<BitbucketCommit>>(0)
			.await?
			.into_iter()
			.flat_map(|v| v.values)
			.map(|v| Box::new(v) as Box<dyn RemoteCommit>)
			.collect())
	}

	/// Fetches the Bitbucket API and returns the pull requests.
	pub async fn get_pull_requests(
		&self,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		Ok(self
			.fetch_with_early_exit::<BitbucketPagination<BitbucketPullRequest>>(0)
			.await?
			.into_iter()
			.flat_map(|v| v.values)
			.map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
			.collect())
	}
}
