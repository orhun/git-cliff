use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};

use super::*;

/// GitLab REST API url.
const GITLAB_API_URL_CFG: ApiUrlCfg = ApiUrlCfg {
	api_url:        "https://gitlab.com/api/v4",
	env_var:        "GITLAB_API_URL",
	api_path:       &["api", "v4"],
	default_domain: "",
};

/// Representation of a single GitLab Project.
///
/// <https://docs.gitlab.com/ee/api/projects.html#get-single-project>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabProject {
	/// GitLab id for project
	pub id: i64,
}

impl RemoteEntry for GitLabProject {
	fn url(_project_id: i64, api_url: &Url, remote: &Remote, _page: i32) -> Url {
		let mut url = api_url.clone();
		url.path_segments_mut()
			.expect("invalid url")
			.extend(&["projects", &format!("{}%2F{}", remote.owner, remote.repo)]);
		url
	}

	fn buffer_size() -> usize {
		1
	}
}

/// Representation of a single commit.
///
/// <https://docs.gitlab.com/ee/api/commits.html>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabCommit {
	/// Sha
	pub id:                    String,
	/// Author
	pub author_name:           String,
	pub(crate) committed_date: String,
}

impl From<GitLabCommit> for RemoteCommit {
	fn from(value: GitLabCommit) -> Self {
		Self {
			id:       value.id,
			username: Some(value.author_name),
		}
	}
}

impl RemoteEntry for GitLabCommit {
	fn url(id: i64, api_url: &Url, _remote: &Remote, page: i32) -> Url {
        let mut url = api_url.clone();
        let commit_page = page + 1;
		url.path_segments_mut().expect("invalid url").extend(&[
			"projects",
			&id.to_string(),
			"repository",
			"commits",
		]);
		url.query_pairs_mut()
			.append_pair("per_page", MAX_PAGE_SIZE)
			.append_pair("page", &commit_page.to_string());
        url
	}
	fn buffer_size() -> usize {
		10
	}
}

/// Representation of a single pull request.
///
/// <https://docs.gitlab.com/ee/api/merge_requests.html>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabMergeRequest {
	/// Numeric project-wide ID
	pub iid:              i64,
	/// Title
	pub title:            String,
	/// Merge Commit Sha
	pub merge_commit_sha: Option<String>,
	/// Labels
	pub labels:           Vec<String>,
}

impl From<GitLabMergeRequest> for RemotePullRequest {
	fn from(value: GitLabMergeRequest) -> Self {
		Self {
			number:       value.iid,
			title:        Some(value.title),
			labels:       value.labels,
			merge_commit: value.merge_commit_sha,
		}
	}
}

impl RemoteEntry for GitLabMergeRequest {
	fn url(id: i64, api_url: &Url, _remote: &Remote, page: i32) -> Url {
        let mut url = api_url.clone();
		url.path_segments_mut().expect("invalid url").extend(&[
			"projects",
			&id.to_string(),
			"merge_requests",
		]);
		url.query_pairs_mut()
			.append_pair("per_page", MAX_PAGE_SIZE)
			.append_pair("page", &page.to_string())
            .append_pair("state", "merged");
        url
	}

	fn buffer_size() -> usize {
		5
	}
}

/// HTTP client for handling GitLab REST API requests.
#[derive(Debug, Clone)]
pub struct GitLabClient {
	/// Remote.
	remote:     Remote,
	/// GitLab API Url
	api_url:    Url,
	/// GitLab project ID
	project_id: i64,
	/// HTTP client.
	client:     ClientWithMiddleware,
}

/// Constructs a GitLab client from the remote configuration.
impl TryFrom<Remote> for GitLabClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		Ok(Self {
			client: create_remote_client(&remote, "application/json")?,
			api_url: GITLAB_API_URL_CFG.get_api_url(&remote)?,
			project_id: 0,
			remote,
		})
	}
}

impl RemoteClientInternal for GitLabClient {
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
impl RemoteClient for GitLabClient {
	async fn init(&mut self) -> Result<()> {
		let project = self.get_entry::<GitLabProject>(0, 0).await?;
		self.project_id = project.id;
		Ok(())
	}

	async fn get_commits(&self) -> Result<Vec<RemoteCommit>> {
		Ok(self
			.fetch::<GitLabCommit>(self.project_id)
			.await?
			.into_iter()
			.map(RemoteCommit::from)
			.collect())
	}

	async fn get_pull_requests(&self) -> Result<Vec<RemotePullRequest>> {
		Ok(self
			.fetch::<GitLabMergeRequest>(self.project_id)
			.await?
			.into_iter()
			.map(RemotePullRequest::from)
			.collect())
	}
}
