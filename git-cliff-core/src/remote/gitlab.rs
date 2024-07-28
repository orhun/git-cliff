use crate::config::Remote;
use crate::error::*;
use reqwest_middleware::ClientWithMiddleware;
use serde::{
	Deserialize,
	Serialize,
};
use std::env;

use super::*;

/// GitLab REST API url.
const GITLAB_API_URL: &str = "https://gitlab.com/api/v4";

/// Environment variable for overriding the GitLab REST API url.
const GITLAB_API_URL_ENV: &str = "GITLAB_API_URL";

/// Log message to show while fetching data from GitLab.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitLab...";

/// Log message to show when done fetching from GitLab.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitLab data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["gitlab", "commit.gitlab"];

/// Representation of a single GitLab Project.
///
/// <https://docs.gitlab.com/ee/api/projects.html#get-single-project>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabProject {
	/// GitLab id for project
	pub id:                  i64,
	/// Optional Description of project
	pub description:         Option<String>,
	/// Name of project
	pub name:                String,
	/// Name of project with namespace owner / repo
	pub name_with_namespace: String,
	/// Name of project with namespace owner/repo
	pub path_with_namespace: String,
	/// Project created at
	pub created_at:          String,
	/// Default branch eg (main/master)
	pub default_branch:      String,
}

impl RemoteEntry for GitLabProject {
	fn url(_id: i64, api_url: &str, remote: &Remote, _page: i32) -> String {
		format!(
			"{}/projects/{}%2F{}",
			api_url,
			urlencoding::encode(remote.owner.as_str()),
			remote.repo
		)
	}

	fn buffer_size() -> usize {
		1
	}

	fn early_exit(&self) -> bool {
		false
	}
}

/// Representation of a single commit.
///
/// <https://docs.gitlab.com/ee/api/commits.html>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabCommit {
	/// Sha
	pub id:              String,
	/// Short Sha
	pub short_id:        String,
	/// Git message
	pub title:           String,
	/// Author
	pub author_name:     String,
	/// Author Email
	pub author_email:    String,
	/// Authored Date
	pub authored_date:   String,
	/// Committer Name
	pub committer_name:  String,
	/// Committer Email
	pub committer_email: String,
	/// Committed Date
	pub committed_date:  String,
	/// Created At
	pub created_at:      String,
	/// Git Message
	pub message:         String,
	/// Parent Ids
	pub parent_ids:      Vec<String>,
	/// Web Url
	pub web_url:         String,
}

impl RemoteCommit for GitLabCommit {
	fn id(&self) -> String {
		self.id.clone()
	}

	fn username(&self) -> Option<String> {
		Some(self.author_name.clone())
	}
}

impl RemoteEntry for GitLabCommit {
	fn url(id: i64, api_url: &str, _remote: &Remote, page: i32) -> String {
		let commit_page = page + 1;
		format!(
			"{}/projects/{}/repository/commits?per_page={MAX_PAGE_SIZE}&\
			 page={commit_page}",
			api_url, id
		)
	}
	fn buffer_size() -> usize {
		10
	}

	fn early_exit(&self) -> bool {
		false
	}
}

/// Representation of a single pull request.
///
/// <https://docs.gitlab.com/ee/api/merge_requests.html>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabMergeRequest {
	/// Id
	pub id:                i64,
	/// Iid
	pub iid:               i64,
	/// ProjectId
	pub project_id:        i64,
	/// Title
	pub title:             String,
	/// Description
	pub description:       String,
	/// State
	pub state:             String,
	/// Created At
	pub created_at:        String,
	/// Author
	pub author:            GitLabUser,
	/// Commit Sha
	pub sha:               String,
	/// Merge Commit Sha
	pub merge_commit_sha:  Option<String>,
	/// Squash Commit Sha
	pub squash_commit_sha: Option<String>,
	/// WebUrl
	pub web_url:           String,
	/// Labels
	pub labels:            Vec<String>,
}

impl RemotePullRequest for GitLabMergeRequest {
	fn number(&self) -> i64 {
		self.iid
	}

	fn title(&self) -> Option<String> {
		Some(self.title.clone())
	}

	fn labels(&self) -> Vec<String> {
		self.labels.clone()
	}

	fn merge_commit(&self) -> Option<String> {
		self.merge_commit_sha.clone()
	}
}

impl RemoteEntry for GitLabMergeRequest {
	fn url(id: i64, api_url: &str, _remote: &Remote, page: i32) -> String {
		format!(
			"{}/projects/{}/merge_requests?per_page={MAX_PAGE_SIZE}&page={page}&\
			 state=merged",
			api_url, id
		)
	}

	fn buffer_size() -> usize {
		5
	}

	fn early_exit(&self) -> bool {
		false
	}
}

/// Representation of a GitLab User.
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabUser {
	/// Id
	pub id:         i64,
	/// Name
	pub name:       String,
	/// Username
	pub username:   String,
	/// State of the User
	pub state:      String,
	/// Url for avatar
	pub avatar_url: Option<String>,
	/// Web Url
	pub web_url:    String,
}

/// Representation of a GitLab Reference.
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabReference {
	/// Short id
	pub short:    String,
	/// Relative Link
	pub relative: String,
	/// Full Link
	pub full:     String,
}

/// HTTP client for handling GitLab REST API requests.
#[derive(Debug, Clone)]
pub struct GitLabClient {
	/// Remote.
	remote: Remote,
	/// HTTP client.
	client: ClientWithMiddleware,
}

/// Constructs a GitLab client from the remote configuration.
impl TryFrom<Remote> for GitLabClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		Ok(Self {
			client: create_remote_client(&remote, "application/json")?,
			remote,
		})
	}
}

impl RemoteClient for GitLabClient {
	fn api_url() -> String {
		env::var(GITLAB_API_URL_ENV)
			.ok()
			.unwrap_or_else(|| GITLAB_API_URL.to_string())
	}

	fn remote(&self) -> Remote {
		self.remote.clone()
	}

	fn client(&self) -> ClientWithMiddleware {
		self.client.clone()
	}
}

impl GitLabClient {
	/// Fetches the GitLab API and returns the pull requests.
	pub async fn get_project(&self) -> Result<GitLabProject> {
		self.get_entry::<GitLabProject>(0, 1).await
	}

	/// Fetches the GitLab API and returns the commits.
	pub async fn get_commits(
		&self,
		project_id: i64,
	) -> Result<Vec<Box<dyn RemoteCommit>>> {
		Ok(self
			.fetch::<GitLabCommit>(project_id)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemoteCommit>)
			.collect())
	}

	/// Fetches the GitLab API and returns the pull requests.
	pub async fn get_merge_requests(
		&self,
		project_id: i64,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		Ok(self
			.fetch::<GitLabMergeRequest>(project_id)
			.await?
			.into_iter()
			.map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
			.collect())
	}
}
#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn gitlab_remote_encodes_owner() {
		let remote = Remote::new("abc/def", "xyz1");
		assert_eq!(
			"https://gitlab.test.com/api/v4/projects/abc%2Fdef%2Fxyz1",
			GitLabProject::url(1, "https://gitlab.test.com/api/v4", &remote, 0)
		)
	}
}
