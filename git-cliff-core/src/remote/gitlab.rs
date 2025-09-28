use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::*;
use crate::config::Remote;
use crate::error::*;

/// Log message to show while fetching data from GitLab.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitLab...";

/// Log message to show when done fetching from GitLab.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitLab data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["gitlab", "commit.gitlab", "commit.remote"];

/// Representation of a single GitLab Project.
///
/// <https://docs.gitlab.com/ee/api/projects.html#get-single-project>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabProject {
    /// GitLab id for project
    pub id: i64,
    /// Optional Description of project
    pub description: Option<String>,
    /// Name of project
    pub name: String,
    /// Name of project with namespace owner / repo
    pub name_with_namespace: String,
    /// Name of project with namespace owner/repo
    pub path_with_namespace: String,
    /// Project created at
    pub created_at: String,
    /// Default branch eg (main/master)
    pub default_branch: Option<String>,
}

/// Representation of a single commit.
///
/// <https://docs.gitlab.com/ee/api/commits.html>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabCommit {
    /// Sha
    pub id: String,
    /// Short Sha
    pub short_id: String,
    /// Git message
    pub title: String,
    /// Author
    pub author_name: String,
    /// Author Email
    pub author_email: String,
    /// Authored Date
    pub authored_date: String,
    /// Committer Name
    pub committer_name: String,
    /// Committer Email
    pub committer_email: String,
    /// Committed Date
    pub committed_date: String,
    /// Created At
    pub created_at: String,
    /// Git Message
    pub message: String,
    /// Parent Ids
    pub parent_ids: Vec<String>,
    /// Web Url
    pub web_url: String,
}

impl RemoteCommit for GitLabCommit {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn username(&self) -> Option<String> {
        Some(self.author_name.clone())
    }

    fn timestamp(&self) -> Option<i64> {
        Some(self.convert_to_unix_timestamp(self.committed_date.clone().as_str()))
    }
}

/// Representation of a single pull request.
///
/// <https://docs.gitlab.com/ee/api/merge_requests.html>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabMergeRequest {
    /// Id
    pub id: i64,
    /// Iid
    pub iid: i64,
    /// Project Id
    pub project_id: i64,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// State
    pub state: String,
    /// Created At
    pub created_at: String,
    /// Author
    pub author: GitLabUser,
    /// Commit Sha
    pub sha: String,
    /// Merge Commit Sha
    pub merge_commit_sha: Option<String>,
    /// Squash Commit Sha
    pub squash_commit_sha: Option<String>,
    /// Web Url
    pub web_url: String,
    /// Labels
    pub labels: Vec<String>,
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
        self.merge_commit_sha.clone().or_else(|| {
            self.squash_commit_sha
                .clone()
                .or_else(|| Some(self.sha.clone()))
        })
    }
}

/// Representation of a GitLab User.
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabUser {
    /// Id
    pub id: i64,
    /// Name
    pub name: String,
    /// Username
    pub username: String,
    /// State of the User
    pub state: String,
    /// Url for avatar
    pub avatar_url: Option<String>,
    /// Web Url
    pub web_url: String,
}

/// Representation of a GitLab Reference.
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabReference {
    /// Short id
    pub short: String,
    /// Relative Link
    pub relative: String,
    /// Full Link
    pub full: String,
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
            client: remote.create_client("application/json")?,
            remote,
        })
    }
}

impl RemoteClient for GitLabClient {
    const API_URL: &'static str = "https://gitlab.com/api/v4";
    const API_URL_ENV: &'static str = "GITLAB_API_URL";

    fn remote(&self) -> Remote {
        self.remote.clone()
    }

    fn client(&self) -> ClientWithMiddleware {
        self.client.clone()
    }

    fn commits_stream<'a>(&'a self) -> impl Stream<Item = Result<Box<dyn RemoteCommit>>> + 'a {
        async_stream! {
            match self.get_project(None).await {
                Ok(project) => {
                    let project_id = project.id;

                    let page_stream = stream::iter(0..)
                        .map(move |page| async move {
                            let gitlab_page = page + 1;
                            let url = Self::commits_url(&self.api_url(), project_id, gitlab_page);
                            self.get_json::<Vec<GitLabCommit>>(&url).await
                        })
                        .buffered(10);

                    let mut page_stream = Box::pin(page_stream);

                    while let Some(page_result) = page_stream.next().await {
                        match page_result {
                            Ok(commits) => {
                                if commits.is_empty() {
                                    break;
                                }

                                for commit in commits {
                                    yield Ok(Box::new(commit) as Box<dyn RemoteCommit>);
                                }
                            }
                            Err(e) => {
                                if let Error::PaginationError(_) = e {
                                    break; // End of pages
                                }
                                yield Err(e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    yield Err(e);
                }
            }
        }
    }

    fn pull_requests_stream<'a>(
        &'a self,
    ) -> impl Stream<Item = Result<Box<dyn RemotePullRequest>>> + 'a {
        async_stream! {
            match self.get_project(None).await {
                Ok(project) => {
                    let project_id = project.id;

                    let page_stream = stream::iter(0..)
                        .map(move |page| async move {
                            let gitlab_page = page + 1;
                            let url = Self::merge_requests_url(&self.api_url(), project_id, gitlab_page);
                            self.get_json::<Vec<GitLabMergeRequest>>(&url).await
                        })
                        .buffered(5);

                    let mut page_stream = Box::pin(page_stream);

                    while let Some(page_result) = page_stream.next().await {
                        match page_result {
                            Ok(mrs) => {
                                if mrs.is_empty() {
                                    break;
                                }

                                for mr in mrs {
                                    yield Ok(Box::new(mr) as Box<dyn RemotePullRequest>);
                                }
                            }
                            Err(e) => {
                                if let Error::PaginationError(_) = e {
                                    break; // End of pages
                                }
                                yield Err(e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    yield Err(e);
                }
            }
        }
    }
}

impl GitLabClient {
    /// Constructs the URL for GitLab project API.
    fn project_url(api_url: &str, owner: &str, repo: &str) -> String {
        format!(
            "{}/projects/{}%2F{}",
            api_url,
            urlencoding::encode(owner),
            repo
        )
    }

    /// Constructs the URL for GitLab commits API.
    fn commits_url(api_url: &str, project_id: i64, page: i32) -> String {
        format!(
            "{}/projects/{}/repository/commits?page={}&per_page={MAX_PAGE_SIZE}",
            api_url, project_id, page
        )
    }

    /// Constructs the URL for GitLab merge requests API.
    fn merge_requests_url(api_url: &str, project_id: i64, page: i32) -> String {
        format!(
            "{}/projects/{}/merge_requests?page={}&per_page={MAX_PAGE_SIZE}&state=merged",
            api_url, project_id, page
        )
    }

    /// Fetches the GitLab API and returns the pull requests.
    pub async fn get_project(&self, _ref_name: Option<&str>) -> Result<GitLabProject> {
        let url = Self::project_url(&self.api_url(), &self.remote().owner, &self.remote().repo);
        self.get_json::<GitLabProject>(&url).await
    }

    /// Fetches the GitLab API and returns the commits.
    pub async fn get_commits(
        &self,
        _project_id: i64,
        _ref_name: Option<&str>,
    ) -> Result<Vec<Box<dyn RemoteCommit>>> {
        use futures::TryStreamExt;
        self.commits_stream().try_collect().await
    }

    /// Fetches the GitLab API and returns the pull requests.
    pub async fn get_merge_requests(
        &self,
        _project_id: i64,
        _ref_name: Option<&str>,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        use futures::TryStreamExt;
        self.pull_requests_stream().try_collect().await
    }
}
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn gitlab_project_url_encodes_owner() {
        let url = GitLabClient::project_url("https://gitlab.test.com/api/v4", "abc/def", "xyz1");
        assert_eq!(
            "https://gitlab.test.com/api/v4/projects/abc%2Fdef%2Fxyz1",
            url
        );
    }

    #[test]
    fn timestamp() {
        let remote_commit = GitLabCommit {
            id: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
            author_name: String::from("orhun"),
            committed_date: String::from("2021-07-18T15:14:39+03:00"),
            ..Default::default()
        };

        assert_eq!(Some(1626610479), remote_commit.timestamp());
    }

    #[test]
    fn merge_request_no_merge_commit() {
        let mr = GitLabMergeRequest {
            sha: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
            ..Default::default()
        };
        assert!(mr.merge_commit().is_some());
    }

    #[test]
    fn merge_request_squash_commit() {
        let mr = GitLabMergeRequest {
            squash_commit_sha: Some(String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071")),
            ..Default::default()
        };
        assert!(mr.merge_commit().is_some());
    }
}
