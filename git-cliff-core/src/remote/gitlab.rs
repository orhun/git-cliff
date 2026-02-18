use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::{Debug, MAX_PAGE_SIZE, RemoteClient, RemoteCommit, RemotePullRequest};
use crate::config::Remote;
use crate::error::{Error, Result};

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["gitlab", "commit.gitlab", "commit.remote"];

/// Representation of a single GitLab Project.
///
/// <https://docs.gitlab.com/ee/api/projects.html#get-single-project>
/// <https://gitlab.com/gitlab-org/gitlab/-/blob/master/doc/api/openapi/openapi.yaml>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabProject {
    /// GitLab id for project
    pub id: Option<i64>,
    /// Optional Description of project
    pub description: Option<String>,
    /// Name of project
    pub name: Option<String>,
    /// Name of project with namespace owner / repo
    pub name_with_namespace: Option<String>,
    /// Name of project with namespace owner/repo
    pub path_with_namespace: Option<String>,
    /// Project created at
    pub created_at: Option<String>,
    /// Default branch eg (main/master)
    pub default_branch: Option<String>,
}

/// Representation of a single commit.
///
/// <https://docs.gitlab.com/ee/api/commits.html>
/// <https://gitlab.com/gitlab-org/gitlab/-/blob/master/doc/api/openapi/openapi.yaml>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabCommit {
    /// Sha
    pub id: Option<String>,
    /// Short Sha
    pub short_id: Option<String>,
    /// Git message
    pub title: Option<String>,
    /// Author
    pub author_name: Option<String>,
    /// Author Email
    pub author_email: Option<String>,
    /// Authored Date
    pub authored_date: Option<String>,
    /// Committer Name
    pub committer_name: Option<String>,
    /// Committer Email
    pub committer_email: Option<String>,
    /// Committed Date
    pub committed_date: Option<String>,
    /// Created At
    pub created_at: Option<String>,
    /// Git Message
    pub message: Option<String>,
    /// Parent Ids
    pub parent_ids: Vec<String>,
    /// Web Url
    pub web_url: Option<String>,
}

impl RemoteCommit for GitLabCommit {
    fn id(&self) -> String {
        self.id
            .clone()
            .expect("Commit id is required for git-cliff semantics")
    }

    fn username(&self) -> Option<String> {
        self.author_name.clone()
    }

    fn timestamp(&self) -> Option<i64> {
        self.committed_date
            .as_deref()
            .map(|d| self.convert_to_unix_timestamp(d))
    }
}

/// Representation of a single pull request.
///
/// <https://docs.gitlab.com/ee/api/merge_requests.html>
/// <https://gitlab.com/gitlab-org/gitlab/-/blob/master/doc/api/openapi/openapi.yaml>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitLabMergeRequest {
    /// Id
    pub id: Option<i64>,
    /// Iid
    pub iid: Option<i64>,
    /// Project Id
    pub project_id: Option<i64>,
    /// Title
    pub title: Option<String>,
    /// Description
    pub description: Option<String>,
    /// State
    pub state: Option<String>,
    /// Created At
    pub created_at: Option<String>,
    /// Author
    pub author: Option<GitLabUser>,
    /// Commit Sha
    pub sha: Option<String>,
    /// Merge Commit Sha
    pub merge_commit_sha: Option<String>,
    /// Squash Commit Sha
    pub squash_commit_sha: Option<String>,
    /// Web Url
    pub web_url: Option<String>,
    /// Labels
    pub labels: Vec<String>,
}

impl RemotePullRequest for GitLabMergeRequest {
    fn number(&self) -> i64 {
        self.iid
            .expect("Merge request id is required for git-cliff semantics")
    }

    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn labels(&self) -> Vec<String> {
        self.labels.clone()
    }

    fn merge_commit(&self) -> Option<String> {
        self.merge_commit_sha
            .clone()
            .or_else(|| self.squash_commit_sha.clone().or_else(|| self.sha.clone()))
    }
}

/// Representation of a GitLab User.
///
/// <https://gitlab.com/gitlab-org/gitlab/-/blob/master/doc/api/openapi/openapi.yaml>
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabUser {
    /// Id
    pub id: Option<i64>,
    /// Name
    pub name: Option<String>,
    /// Username
    pub username: Option<String>,
    /// State of the User
    pub state: Option<String>,
    /// Url for avatar
    pub avatar_url: Option<String>,
    /// Web Url
    pub web_url: Option<String>,
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
}

impl GitLabClient {
    /// Constructs the URL for GitLab project API.
    fn project_url(api_url: &str, remote: &Remote) -> String {
        format!(
            "{}/projects/{}%2F{}",
            api_url,
            urlencoding::encode(remote.owner.as_str()),
            remote.repo
        )
    }

    /// Constructs the URL for GitLab commits API.
    fn commits_url(project_id: i64, api_url: &str, ref_name: Option<&str>, page: i32) -> String {
        let mut url = format!(
            "{api_url}/projects/{project_id}/repository/commits?per_page={MAX_PAGE_SIZE}&\
             page={page}"
        );

        if let Some(ref_name) = ref_name {
            url.push_str(&format!("&ref_name={ref_name}"));
        }

        url
    }
    /// Constructs the URL for GitLab merge requests API.
    fn pull_requests_url(project_id: i64, api_url: &str, page: i32) -> String {
        format!(
            "{api_url}/projects/{project_id}/merge_requests?per_page={MAX_PAGE_SIZE}&page={page}&\
             state=merged"
        )
    }

    /// Looks up the project details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_project(&self) -> Result<GitLabProject> {
        let url = Self::project_url(&self.api_url(), &self.remote());
        self.get_json::<GitLabProject>(&url).await
    }

    /// Fetches the complete list of commits.
    /// This is inefficient for large repositories; consider using
    /// `get_commit_stream` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_commits(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> Result<Vec<Box<dyn RemoteCommit>>> {
        use futures::TryStreamExt;
        self.get_commit_stream(project_id, ref_name)
            .try_collect()
            .await
    }

    /// Fetches the complete list of pull requests.
    /// This is inefficient for large repositories; consider using
    /// `get_pull_request_stream` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_pull_requests(
        &self,
        project_id: i64,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        use futures::TryStreamExt;
        self.get_pull_request_stream(project_id).try_collect().await
    }

    fn get_commit_stream(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> impl Stream<Item = Result<Box<dyn RemoteCommit>>> + '_ {
        let ref_name = ref_name.map(ToString::to_string);
        async_stream! {
                // GitLab pages are 1-indexed
                let page_stream = stream::iter(1..)
                    .map(move |page| {
                        let ref_name = ref_name.clone();
                        async move {
                            let url = Self::commits_url(project_id, &self.api_url(), ref_name.as_deref(), page);
                            self.get_json::<Vec<GitLabCommit>>(&url).await
                        }
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
                            yield Err(e);
                            break;
                        }
                    }
                }
        }
    }

    fn get_pull_request_stream(
        &self,
        project_id: i64,
    ) -> impl Stream<Item = Result<Box<dyn RemotePullRequest>>> + '_ {
        async_stream! {
            // GitLab pages are 1-indexed
            let page_stream = stream::iter(1..)
                .map(move |page| async move {
                    let url = Self::pull_requests_url(project_id, &self.api_url(), page);
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
                        yield Err(e);
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn gitlab_project_url_encodes_owner() {
        let remote = Remote {
            owner: "abc/def".to_string(),
            repo: "xyz1".to_string(),
            ..Default::default()
        };
        let url = GitLabClient::project_url("https://gitlab.test.com/api/v4", &remote);
        assert_eq!(
            "https://gitlab.test.com/api/v4/projects/abc%2Fdef%2Fxyz1",
            url
        );
    }

    #[test]
    fn timestamp() {
        let remote_commit = GitLabCommit {
            id: Some(String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071")),
            author_name: Some(String::from("orhun")),
            committed_date: Some(String::from("2021-07-18T15:14:39+03:00")),
            ..Default::default()
        };

        assert_eq!(Some(1_626_610_479), remote_commit.timestamp());
    }

    #[test]
    fn pull_request_no_merge_commit() {
        let mr = GitLabMergeRequest {
            sha: Some(String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071")),
            ..Default::default()
        };
        assert!(mr.merge_commit().is_some());
    }

    #[test]
    fn pull_request_squash_commit() {
        let mr = GitLabMergeRequest {
            squash_commit_sha: Some(String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071")),
            ..Default::default()
        };
        assert!(mr.merge_commit().is_some());
    }
}
