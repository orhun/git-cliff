use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};

use super::*;

/// Bitbucket REST API url.
const BITBUCKET_API_URL_CFG: ApiUrlCfg = ApiUrlCfg {
	api_url:        "https://api.bitbucket.org/2.0",
	env_var:        "BITBUCKET_API_URL",
	// Self-hosted Bitbucket Server API path
	api_path:       &["rest", "api", "2.0"],
	default_domain: "bitbucket.org",
};

/// Maximum number of entries to fetch for bitbucket pull requests.
const BITBUCKET_MAX_PAGE_PRS: &str = "50";

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketCommit {
	/// SHA.
	pub hash:   String,
	/// Author of the commit.
	pub author: Option<BitbucketCommitAuthor>,
}

impl From<BitbucketCommit> for RemoteCommit {
	fn from(value: BitbucketCommit) -> Self {
		Self {
			id:       value.hash,
			username: value.author.and_then(|a| a.login),
		}
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-commits/#api-repositories-workspace-repo-slug-commits-get>
impl RemoteEntry for BitbucketPagination<BitbucketCommit> {
	fn url(_id: i64, api_url: &Url, remote: &Remote, page: i32) -> Url {
		let mut url = api_url.clone();
		let commit_page = page + 1;
		url.path_segments_mut().expect("invalid url").extend(&[
			"repositories",
			&remote.owner,
			&remote.repo,
			"commits",
		]);
		url.query_pairs_mut()
			.append_pair("pagelen", MAX_PAGE_SIZE)
			.append_pair("page", &commit_page.to_string());
		url
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
	pub id:           i64,
	/// Pull request title.
	pub title:        Option<String>,
	/// Bitbucket Pull Request Merge Commit
	pub merge_commit: Option<BitbucketPullRequestMergeCommit>,
}

impl From<BitbucketPullRequest> for RemotePullRequest {
	fn from(value: BitbucketPullRequest) -> Self {
		Self {
			number:       value.id,
			title:        value.title,
			labels:       Vec::new(),
			merge_commit: value.merge_commit.map(|c| c.hash),
		}
	}
}

/// <https://developer.atlassian.com/cloud/bitbucket/rest/api-group-pullrequests/#api-repositories-workspace-repo-slug-pullrequests-get>
impl RemoteEntry for BitbucketPagination<BitbucketPullRequest> {
	fn url(_id: i64, api_url: &Url, remote: &Remote, page: i32) -> Url {
		let mut url = api_url.clone();
		let pr_page = page + 1;
		url.path_segments_mut().expect("invalid url").extend(&[
			"repositories",
			&remote.owner,
			&remote.repo,
			"pullrequests",
		]);
		url.query_pairs_mut()
			.append_pair("pagelen", BITBUCKET_MAX_PAGE_PRS)
			.append_pair("page", &pr_page.to_string())
			.append_pair("state", "MERGED");
		url
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
	remote:  Remote,
	/// Bitbucket API Url
	api_url: Url,
	/// HTTP client.
	client:  ClientWithMiddleware,
}

/// Constructs a Bitbucket client from the remote configuration.
impl TryFrom<Remote> for BitbucketClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		Ok(Self {
			client: create_remote_client(&remote, "application/json")?,
			api_url: BITBUCKET_API_URL_CFG.get_api_url(&remote)?,
			remote,
		})
	}
}

impl RemoteClientInternal for BitbucketClient {
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
impl RemoteClient for BitbucketClient {
	async fn get_commits(&self) -> Result<Vec<RemoteCommit>> {
		Ok(self
			.fetch_with_early_exit::<BitbucketPagination<BitbucketCommit>>(0)
			.await?
			.into_iter()
			.flat_map(|v| v.values)
			.map(RemoteCommit::from)
			.collect())
	}

	async fn get_pull_requests(&self) -> Result<Vec<RemotePullRequest>> {
		Ok(self
			.fetch_with_early_exit::<BitbucketPagination<BitbucketPullRequest>>(0)
			.await?
			.into_iter()
			.flat_map(|v| v.values)
			.map(RemotePullRequest::from)
			.collect())
	}
}
