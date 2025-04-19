use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};

use super::*;

/// Log message to show while fetching data from GitHub.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitHub...";

/// Log message to show when done fetching from GitHub.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitHub data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] =
	&["github", "commit.github", "commit.remote"];

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommit {
	/// SHA.
	pub sha:    String,
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

impl RemoteEntry for GitHubCommit {
	fn url(
		_id: i64,
		api_url: &str,
		remote: &Remote,
		ref_name: Option<&str>,
		page: i32,
	) -> String {
		let mut url = format!(
			"{}/repos/{}/{}/commits?per_page={MAX_PAGE_SIZE}&page={page}",
			api_url, remote.owner, remote.repo
		);

		if let Some(ref_name) = ref_name {
			url.push_str(&format!("&sha={}", ref_name));
		}

		url
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
	fn url(
		_id: i64,
		api_url: &str,
		remote: &Remote,
		_ref_name: Option<&str>,
		page: i32,
	) -> String {
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
	/// Fetches the GitHub API and returns the commits.
	pub async fn get_commits(
		&self,
		ref_name: Option<&str>,
	) -> Result<Vec<Box<dyn RemoteCommit>>> {
		Ok(self
			.fetch::<GitHubCommit>(0, ref_name)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemoteCommit>)
			.collect())
	}

	/// Fetches the GitHub API and returns the pull requests.
	pub async fn get_pull_requests(
		&self,
		ref_name: Option<&str>,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		Ok(self
			.fetch::<GitHubPullRequest>(0, ref_name)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
			.collect())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::remote::RemoteCommit;
	use pretty_assertions::assert_eq;

	#[test]
	fn timestamp() {
		let remote_commit = GitHubCommit {
			sha:    String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
			author: Some(GitHubCommitAuthor {
				login: Some(String::from("orhun")),
			}),
			commit: Some(GitHubCommitDetails {
				author: GitHubCommitDetailsAuthor {
					date: String::from("2021-07-18T15:14:39+03:00"),
				},
			}),
		};

		assert_eq!(Some(1626610479), remote_commit.timestamp());
	}
}
