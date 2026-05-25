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
    /// Resolved GitLab username.
    ///
    /// The commits API only returns `author_name`, which is the display name.
    /// This field is populated by resolving the author via merge requests and
    /// project members.
    #[serde(default, skip)]
    pub resolved_username: Option<String>,
}

impl RemoteCommit for GitLabCommit {
    fn id(&self) -> String {
        self.id
            .clone()
            .expect("Commit id is required for git-cliff semantics")
    }

    fn username(&self) -> Option<String> {
        self.resolved_username.clone()
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
    /// Public email of the user.
    pub public_email: Option<String>,
}

/// Representation of a GitLab project member.
///
/// <https://docs.gitlab.com/ee/api/project_members.html#list-all-members-of-a-project>
#[derive(Debug, Default, Clone, Deserialize)]
struct GitLabProjectMember {
    /// Username
    username: Option<String>,
    /// Public email
    public_email: Option<String>,
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

    /// Constructs the URL for GitLab project members API.
    fn members_url(project_id: i64, api_url: &str, query: &str, page: i32) -> String {
        format!(
            "{api_url}/projects/{project_id}/members/all?per_page={MAX_PAGE_SIZE}&page={page}&\
             query={}",
            urlencoding::encode(query)
        )
    }

    /// Constructs the URL for GitLab user search API.
    fn users_search_url(api_url: &str, email: &str) -> String {
        format!(
            "{api_url}/users?per_page={MAX_PAGE_SIZE}&search={}",
            urlencoding::encode(email)
        )
    }

    /// Looks up the project details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_project(&self) -> Result<GitLabProject> {
        crate::set_progress_message!("Fetching the project details from GitLab");
        let url = Self::project_url(&self.api_url(), &self.remote());
        self.get_json::<GitLabProject>(&url).await
    }

    /// Fetches the complete list of commits.
    /// This is inefficient for large repositories; consider using
    /// `fetch_commits` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_commits(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> Result<Vec<Box<dyn RemoteCommit>>> {
        let commits = self.fetch_commits(project_id, ref_name).await?;
        Ok(commits
            .into_iter()
            .map(|commit| Box::new(commit) as Box<dyn RemoteCommit>)
            .collect())
    }

    /// Fetches the complete list of pull requests.
    /// This is inefficient for large repositories; consider using
    /// `fetch_merge_requests` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_pull_requests(
        &self,
        project_id: i64,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        let merge_requests = self.fetch_merge_requests(project_id).await?;
        Ok(merge_requests
            .into_iter()
            .map(|merge_request| Box::new(merge_request) as Box<dyn RemotePullRequest>)
            .collect())
    }

    /// Fetches the complete list of commits.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn fetch_commits(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> Result<Vec<GitLabCommit>> {
        use futures::TryStreamExt;
        crate::set_progress_message!("Fetching all commits from GitLab");
        self.raw_commit_stream(project_id, ref_name)
            .try_collect()
            .await
    }

    /// Fetches the complete list of merge requests.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn fetch_merge_requests(&self, project_id: i64) -> Result<Vec<GitLabMergeRequest>> {
        use futures::TryStreamExt;
        crate::set_progress_message!("Fetching all pull requests from GitLab");
        self.raw_merge_request_stream(project_id)
            .try_collect()
            .await
    }

    /// Resolves GitLab usernames for commits.
    ///
    /// GitLab's commits API returns the author's display name instead of the
    /// username. This function resolves usernames using merge request authors
    /// and project member lookups by email.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn resolve_commit_usernames(
        &self,
        project_id: i64,
        commits: &mut [GitLabCommit],
        merge_requests: &[GitLabMergeRequest],
    ) -> Result<()> {
        use std::collections::{HashMap, HashSet};

        apply_merge_request_usernames(commits, merge_requests);

        let emails: HashSet<String> = commits
            .iter()
            .filter(|commit| commit.resolved_username.is_none())
            .filter_map(|commit| commit.author_email.clone())
            .filter(|email| !email.is_empty())
            .collect();

        if emails.is_empty() {
            return Ok(());
        }

        let mut email_to_username = HashMap::new();
        for email in emails {
            if email_to_username.contains_key(&email) {
                continue;
            }
            if let Some(username) = self.lookup_username_by_email(project_id, &email).await? {
                email_to_username.insert(email, username);
            }
        }

        for commit in commits.iter_mut() {
            if commit.resolved_username.is_some() {
                continue;
            }
            if let Some(email) = &commit.author_email {
                if let Some(username) = email_to_username.get(email) {
                    commit.resolved_username = Some(username.clone());
                }
            }
        }

        Ok(())
    }

    async fn lookup_username_by_email(
        &self,
        project_id: i64,
        email: &str,
    ) -> Result<Option<String>> {
        let url = Self::members_url(project_id, &self.api_url(), email, 1);
        let members: Vec<GitLabProjectMember> = self.get_json(&url).await?;
        if let Some(username) = members
            .iter()
            .find(|member| member.public_email.as_deref() == Some(email))
            .and_then(|member| member.username.clone())
        {
            return Ok(Some(username));
        }

        let users_url = Self::users_search_url(&self.api_url(), email);
        let users: Vec<GitLabUser> = self.get_json(&users_url).await?;
        Ok(users
            .iter()
            .find(|user| user.public_email.as_deref() == Some(email))
            .and_then(|user| user.username.clone()))
    }

    fn raw_commit_stream(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> impl Stream<Item = Result<GitLabCommit>> + '_ {
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
                                yield Ok(commit);
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

    fn raw_merge_request_stream(
        &self,
        project_id: i64,
    ) -> impl Stream<Item = Result<GitLabMergeRequest>> + '_ {
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
                            yield Ok(mr);
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

/// Applies merge request author usernames to matching commits.
fn apply_merge_request_usernames(
    commits: &mut [GitLabCommit],
    merge_requests: &[GitLabMergeRequest],
) {
    use std::collections::HashMap;

    let mut sha_to_username = HashMap::new();
    for merge_request in merge_requests {
        let Some(username) = merge_request
            .author
            .as_ref()
            .and_then(|author| author.username.clone())
        else {
            continue;
        };
        for sha in [
            merge_request.merge_commit_sha.as_deref(),
            merge_request.squash_commit_sha.as_deref(),
            merge_request.sha.as_deref(),
        ] {
            if let Some(sha) = sha {
                sha_to_username.insert(sha.to_string(), username.clone());
            }
        }
    }

    for commit in commits.iter_mut() {
        if let Some(id) = &commit.id {
            if let Some(username) = sha_to_username.get(id) {
                commit.resolved_username = Some(username.clone());
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
            resolved_username: Some(String::from("orhun")),
            committed_date: Some(String::from("2021-07-18T15:14:39+03:00")),
            ..Default::default()
        };

        assert_eq!(Some(1_626_610_479), remote_commit.timestamp());
    }

    #[test]
    fn username_uses_resolved_username() {
        let remote_commit = GitLabCommit {
            author_name: Some(String::from("Nathan Belsterling")),
            resolved_username: Some(String::from("nbelste1")),
            ..Default::default()
        };

        assert_eq!(Some(String::from("nbelste1")), remote_commit.username());
    }

    #[test]
    fn apply_merge_request_usernames_maps_merge_commits() {
        let mut commits = vec![GitLabCommit {
            id: Some(String::from("abc123")),
            author_name: Some(String::from("Display Name")),
            ..Default::default()
        }];
        let merge_requests = vec![GitLabMergeRequest {
            merge_commit_sha: Some(String::from("abc123")),
            author: Some(GitLabUser {
                username: Some(String::from("gitlab_user")),
                ..Default::default()
            }),
            ..Default::default()
        }];

        apply_merge_request_usernames(&mut commits, &merge_requests);

        assert_eq!(
            Some(String::from("gitlab_user")),
            commits[0].resolved_username
        );
    }

    #[test]
    fn members_url_encodes_query() {
        let url =
            GitLabClient::members_url(1, "https://gitlab.test.com/api/v4", "user@example.com", 1);
        assert_eq!(
            "https://gitlab.test.com/api/v4/projects/1/members/all?per_page=100&page=1&query=user%40example.com",
            url
        );
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
