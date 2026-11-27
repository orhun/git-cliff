use std::collections::{HashMap, HashSet};

use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::{Debug, MAX_PAGE_SIZE, RemoteClient, RemoteCommit, RemoteMetadata, RemotePullRequest};
use crate::config::Remote;
use crate::error::{Error, Result};

/// Maximum number of `/projects/:id/users` pages to fetch when resolving commit
/// authors to GitLab usernames.
const MAX_PROJECT_USER_PAGES: i32 = 10;

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

    fn author(&self) -> Option<String> {
        self.author.clone().and_then(|v| v.username)
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

/// Internal wrapper pairing a [`GitLabCommit`] with its resolved GitLab
/// username so [`GitLabCommit`] keeps its existing public fields unchanged.
#[derive(Debug, Clone)]
struct ResolvedGitLabCommit {
    commit: GitLabCommit,
    author_username: Option<String>,
}

impl From<GitLabCommit> for ResolvedGitLabCommit {
    fn from(commit: GitLabCommit) -> Self {
        Self {
            commit,
            author_username: None,
        }
    }
}

impl RemoteCommit for ResolvedGitLabCommit {
    fn id(&self) -> String {
        self.commit.id()
    }

    fn username(&self) -> Option<String> {
        self.author_username
            .clone()
            .or_else(|| self.commit.username())
    }

    fn timestamp(&self) -> Option<i64> {
        self.commit.timestamp()
    }
}

/// Subset of user fields returned by GitLab's `/users` and `/projects/:id/users`
/// endpoints for resolving commit authors to GitLab usernames.
#[derive(Debug, Default, Clone, Deserialize)]
struct GitLabUserLookup {
    name: Option<String>,
    username: Option<String>,
    public_email: Option<String>,
    email: Option<String>,
}

fn non_empty_trimmed(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|s| !s.is_empty())
}

fn insert_unique(map: &mut HashMap<String, Option<String>>, key: String, value: String) {
    match map.get(&key) {
        None => {
            map.insert(key, Some(value));
        }
        Some(Some(existing)) if existing == &value => {}
        Some(_) => {
            map.insert(key, None);
        }
    }
}

fn is_endpoint_unavailable(err: &Error) -> bool {
    matches!(
        err,
        Error::HttpClientError(e)
            if matches!(
                e.status(),
                Some(
                    reqwest::StatusCode::UNAUTHORIZED |
                        reqwest::StatusCode::FORBIDDEN |
                        reqwest::StatusCode::NOT_FOUND |
                        reqwest::StatusCode::TOO_MANY_REQUESTS
                )
            )
    )
}

/// Matches a commit `author_email` against users returned by
/// `GET /users?search=<email>`, requiring an exact case-insensitive match on
/// `email` or `public_email` resolving to a single unique username.
fn match_user_by_email(email: &str, users: &[GitLabUserLookup]) -> Option<String> {
    let exact_matches: HashSet<&str> =
        users
            .iter()
            .filter(|user| {
                non_empty_trimmed(user.email.as_deref())
                    .is_some_and(|e| e.eq_ignore_ascii_case(email)) ||
                    non_empty_trimmed(user.public_email.as_deref())
                        .is_some_and(|e| e.eq_ignore_ascii_case(email))
            })
            .filter_map(|user| non_empty_trimmed(user.username.as_deref()))
            .collect();
    if exact_matches.len() == 1 {
        exact_matches.into_iter().next().map(ToString::to_string)
    } else {
        None
    }
}

/// Resolves commit usernames from already-fetched merge requests without extra
/// API calls while preserving `username` vs `pr_author` separation.
///
/// Only attributes an MR author's username to a matched commit when the commit
/// author matches that MR author's display name or username, then propagates
/// unambiguously learned `author_email -> username` mappings to other commits
/// (including direct commits) by the same author.
fn resolve_from_merge_requests(
    commits: &mut [ResolvedGitLabCommit],
    merge_requests: &[GitLabMergeRequest],
) {
    let mut mr_by_sha: HashMap<&str, &GitLabUser> = HashMap::new();
    for mr in merge_requests {
        if let (Some(sha), Some(author)) = (
            mr.merge_commit_sha
                .as_deref()
                .or(mr.squash_commit_sha.as_deref())
                .or(mr.sha.as_deref()),
            mr.author.as_ref(),
        ) {
            mr_by_sha.insert(sha, author);
        }
    }

    let mut email_to_username: HashMap<String, Option<String>> = HashMap::new();

    for resolved in commits.iter_mut() {
        if resolved.author_username.is_some() {
            continue;
        }
        let Some(commit_id) = resolved.commit.id.as_deref() else {
            continue;
        };
        let short_id = commit_id.get(..12);
        let Some(mr_author) = mr_by_sha
            .get(commit_id)
            .or_else(|| short_id.and_then(|s| mr_by_sha.get(s)))
            .copied()
        else {
            continue;
        };
        let Some(mr_username) = non_empty_trimmed(mr_author.username.as_deref()) else {
            continue;
        };
        let Some(commit_author_name) = non_empty_trimmed(resolved.commit.author_name.as_deref())
        else {
            continue;
        };
        let target = commit_author_name.to_lowercase();
        let matches_mr_author = non_empty_trimmed(mr_author.name.as_deref())
            .is_some_and(|n| n.to_lowercase() == target) ||
            mr_username.to_lowercase() == target;
        if matches_mr_author {
            resolved.author_username = Some(mr_username.to_string());
            if let Some(email) = non_empty_trimmed(resolved.commit.author_email.as_deref()) {
                insert_unique(
                    &mut email_to_username,
                    email.to_lowercase(),
                    mr_username.to_string(),
                );
            }
        }
    }

    if !email_to_username.is_empty() {
        for resolved in commits.iter_mut() {
            if resolved.author_username.is_some() {
                continue;
            }
            if let Some(email) = non_empty_trimmed(resolved.commit.author_email.as_deref()) &&
                let Some(Some(username)) = email_to_username.get(&email.to_lowercase())
            {
                resolved.author_username = Some(username.clone());
            }
        }
    }
}

/// Applies unambiguous email, display-name, and username matches from project
/// users across all unresolved commits.
fn resolve_from_project_users(commits: &mut [ResolvedGitLabCommit], users: &[GitLabUserLookup]) {
    let mut by_email: HashMap<String, Option<String>> = HashMap::new();
    let mut by_name: HashMap<String, Option<String>> = HashMap::new();
    let mut by_username: HashMap<String, Option<String>> = HashMap::new();

    for user in users {
        let Some(username) = non_empty_trimmed(user.username.as_deref()) else {
            continue;
        };
        if let Some(email) = non_empty_trimmed(user.email.as_deref()) {
            insert_unique(&mut by_email, email.to_lowercase(), username.to_string());
        }
        if let Some(public_email) = non_empty_trimmed(user.public_email.as_deref()) {
            insert_unique(
                &mut by_email,
                public_email.to_lowercase(),
                username.to_string(),
            );
        }
        if let Some(name) = non_empty_trimmed(user.name.as_deref()) {
            insert_unique(&mut by_name, name.to_lowercase(), username.to_string());
        }
        insert_unique(
            &mut by_username,
            username.to_lowercase(),
            username.to_string(),
        );
    }

    for resolved in commits.iter_mut() {
        if resolved.author_username.is_some() {
            continue;
        }
        if let Some(email) = non_empty_trimmed(resolved.commit.author_email.as_deref()) &&
            let Some(Some(username)) = by_email.get(&email.to_lowercase())
        {
            resolved.author_username = Some(username.clone());
            continue;
        }
        if let Some(name) = non_empty_trimmed(resolved.commit.author_name.as_deref()) {
            let key = name.to_lowercase();
            if let Some(Some(username)) = by_name.get(&key).or_else(|| by_username.get(&key)) {
                resolved.author_username = Some(username.clone());
            }
        }
    }
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

    /// Constructs the URL for searching GitLab users by email.
    fn users_search_url(api_url: &str, query: &str) -> String {
        format!(
            "{api_url}/users?search={}&per_page={MAX_PAGE_SIZE}",
            urlencoding::encode(query)
        )
    }

    /// Constructs the URL for listing users of a GitLab project.
    fn project_users_url(project_id: i64, api_url: &str, page: i32) -> String {
        format!("{api_url}/projects/{project_id}/users?per_page={MAX_PAGE_SIZE}&page={page}")
    }

    /// Looks up the project details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_project(&self) -> Result<GitLabProject> {
        crate::set_progress_message!("Fetching the project details from GitLab");
        let url = Self::project_url(&self.api_url(), &self.remote());
        self.get_json::<GitLabProject>(&url).await
    }

    /// Fetches commits and merge requests in parallel and resolves GitLab
    /// usernames for the fetched commits.
    ///
    /// When `relevant_commit_ids` is `Some`, `/users?search=<email>` lookups
    /// and `/projects/:id/users` fetching are triggered only for unresolved
    /// commits in that set, while all learned mappings are applied across the
    /// full commit list so `is_first_time` checks remain consistent.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub(crate) async fn get_metadata(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
        relevant_commit_ids: Option<&HashSet<&str>>,
    ) -> Result<RemoteMetadata> {
        let (commits, merge_requests) = tokio::try_join!(
            self.get_gitlab_commits(project_id, ref_name),
            self.get_gitlab_merge_requests(project_id),
        )?;
        let mut resolved: Vec<ResolvedGitLabCommit> = commits
            .into_iter()
            .map(ResolvedGitLabCommit::from)
            .collect();
        self.resolve_commit_usernames(
            project_id,
            &mut resolved,
            &merge_requests,
            relevant_commit_ids,
        )
        .await;
        let commits = resolved
            .into_iter()
            .map(|commit| Box::new(commit) as Box<dyn RemoteCommit>)
            .collect();
        let merge_requests = merge_requests
            .into_iter()
            .map(|mr| Box::new(mr) as Box<dyn RemotePullRequest>)
            .collect();
        Ok((commits, merge_requests))
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
        let mut resolved: Vec<ResolvedGitLabCommit> = self
            .get_gitlab_commits(project_id, ref_name)
            .await?
            .into_iter()
            .map(ResolvedGitLabCommit::from)
            .collect();
        self.resolve_commit_usernames(project_id, &mut resolved, &[], None)
            .await;
        Ok(resolved
            .into_iter()
            .map(|commit| Box::new(commit) as Box<dyn RemoteCommit>)
            .collect())
    }

    /// Fetches the complete list of pull requests.
    /// This is inefficient for large repositories; consider using
    /// `get_pull_request_stream` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_pull_requests(
        &self,
        project_id: i64,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        let merge_requests = self.get_gitlab_merge_requests(project_id).await?;
        Ok(merge_requests
            .into_iter()
            .map(|mr| Box::new(mr) as Box<dyn RemotePullRequest>)
            .collect())
    }

    async fn get_gitlab_commits(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> Result<Vec<GitLabCommit>> {
        use futures::TryStreamExt;
        crate::set_progress_message!("Fetching all commits from GitLab");
        self.get_commit_stream(project_id, ref_name)
            .try_collect()
            .await
    }

    async fn get_gitlab_merge_requests(&self, project_id: i64) -> Result<Vec<GitLabMergeRequest>> {
        use futures::TryStreamExt;
        crate::set_progress_message!("Fetching all pull requests from GitLab");
        self.get_pull_request_stream(project_id).try_collect().await
    }

    async fn resolve_commit_usernames(
        &self,
        project_id: i64,
        commits: &mut [ResolvedGitLabCommit],
        merge_requests: &[GitLabMergeRequest],
        relevant_commit_ids: Option<&HashSet<&str>>,
    ) {
        if commits.is_empty() {
            return;
        }

        let is_relevant = |commit: &ResolvedGitLabCommit| {
            relevant_commit_ids.is_none_or(|ids| {
                commit
                    .commit
                    .id
                    .as_deref()
                    .is_some_and(|id| ids.contains(id))
            })
        };

        resolve_from_merge_requests(commits, merge_requests);

        let mut seen_emails = HashSet::new();
        let unresolved_emails: Vec<String> = commits
            .iter()
            .filter(|c| c.author_username.is_none() && is_relevant(c))
            .filter_map(|c| non_empty_trimmed(c.commit.author_email.as_deref()))
            .filter(|email| seen_emails.insert(email.to_lowercase()))
            .map(ToString::to_string)
            .collect();

        if !unresolved_emails.is_empty() {
            let email_map = self.lookup_usernames_by_email(unresolved_emails).await;
            for resolved in commits.iter_mut() {
                if resolved.author_username.is_some() {
                    continue;
                }
                if let Some(email) = non_empty_trimmed(resolved.commit.author_email.as_deref()) &&
                    let Some(username) = email_map.get(&email.to_lowercase())
                {
                    resolved.author_username = Some(username.clone());
                }
            }
        }

        let has_unresolved_relevant = commits
            .iter()
            .any(|c| c.author_username.is_none() && is_relevant(c));
        if has_unresolved_relevant {
            let project_users = self.get_project_users(project_id).await;
            if !project_users.is_empty() {
                resolve_from_project_users(commits, &project_users);
            }
        }
    }

    async fn lookup_user_by_email(&self, email: &str) -> Result<Option<String>> {
        let url = Self::users_search_url(&self.api_url(), email);
        let users = self.get_json::<Vec<GitLabUserLookup>>(&url).await?;
        Ok(match_user_by_email(email, &users))
    }

    async fn lookup_usernames_by_email(&self, emails: Vec<String>) -> HashMap<String, String> {
        let mut iter = emails.into_iter();
        let Some(first) = iter.next() else {
            return HashMap::new();
        };

        let mut results = HashMap::new();
        match self.lookup_user_by_email(&first).await {
            Ok(Some(username)) => {
                results.insert(first.to_lowercase(), username);
            }
            Ok(None) => {}
            Err(err) if is_endpoint_unavailable(&err) => {
                tracing::debug!("GitLab users API unavailable ({err}); skipping email lookups");
                return results;
            }
            Err(err) => {
                tracing::debug!("Failed to look up GitLab user for email `{first}`: {err}");
            }
        }

        let remaining: Vec<String> = iter.collect();
        if !remaining.is_empty() {
            let mut lookups = stream::iter(remaining)
                .map(|email| async move {
                    let res = self.lookup_user_by_email(&email).await;
                    (email, res)
                })
                .buffer_unordered(10);

            while let Some((email, res)) = lookups.next().await {
                match res {
                    Ok(Some(username)) => {
                        results.insert(email.to_lowercase(), username);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        tracing::debug!("Failed to look up GitLab user for email `{email}`: {err}");
                    }
                }
            }
        }

        results
    }

    /// Fetches project users via `GET /projects/:id/users`, stopping as soon as
    /// a page returns fewer than [`MAX_PAGE_SIZE`] entries or
    /// [`MAX_PROJECT_USER_PAGES`] is reached, and degrading gracefully on error.
    async fn get_project_users(&self, project_id: i64) -> Vec<GitLabUserLookup> {
        let mut all_users = Vec::new();
        for page in 1..=MAX_PROJECT_USER_PAGES {
            let url = Self::project_users_url(project_id, &self.api_url(), page);
            match self.get_json::<Vec<GitLabUserLookup>>(&url).await {
                Ok(page_users) => {
                    let count = page_users.len();
                    all_users.extend(page_users);
                    if count < MAX_PAGE_SIZE as usize {
                        break;
                    }
                }
                Err(err) => {
                    tracing::debug!("Failed to fetch GitLab project users page {page}: {err}");
                    break;
                }
            }
        }
        all_users
    }

    fn get_commit_stream(
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

    fn get_pull_request_stream(
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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::remote::RemotePullRequest;

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

    #[test]
    fn pull_request_author() {
        let merge_request: GitLabMergeRequest = serde_json::from_str(
            r#"{
                "iid": 42,
                "title": "feat: add pr_author",
                "merge_commit_sha": "1d244937ee6ceb8e0314a4a201ba93a7a61f2071",
                "labels": [],
                "author": { "username": "contributor" }
            }"#,
        )
        .expect("failed to deserialize merge request");

        assert_eq!(Some(String::from("contributor")), merge_request.author());
    }

    #[test]
    fn merge_request_author_missing() {
        let merge_request: GitLabMergeRequest = serde_json::from_str(
            r#"{
                "iid": 42,
                "title": "feat: add pr_author",
                "merge_commit_sha": "1d244937ee6ceb8e0314a4a201ba93a7a61f2071",
                "labels": []
            }"#,
        )
        .expect("failed to deserialize merge request");

        assert_eq!(None, merge_request.author());
    }

    #[test]
    fn resolved_commit_prefers_author_username_with_author_name_fallback() {
        let base = GitLabCommit {
            id: Some(String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071")),
            author_name: Some(String::from("Nathan Belsterling")),
            ..Default::default()
        };
        let resolved = ResolvedGitLabCommit {
            commit: base.clone(),
            author_username: Some(String::from("nbelste1")),
        };
        assert_eq!(Some(String::from("nbelste1")), resolved.username());
        assert_eq!(
            Some(String::from("Nathan Belsterling")),
            ResolvedGitLabCommit::from(base).username()
        );
    }

    #[test]
    fn resolve_from_merge_requests_preserves_pr_author_separation() {
        let mut commits = vec![
            ResolvedGitLabCommit::from(GitLabCommit {
                id: Some(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")),
                author_name: Some(String::from("Flapjack McButterpants")),
                author_email: Some(String::from("flapjack@example.com")),
                ..Default::default()
            }),
            ResolvedGitLabCommit::from(GitLabCommit {
                id: Some(String::from("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")),
                author_name: Some(String::from("Flapjack")),
                author_email: Some(String::from("FLAPJACK@example.com")),
                ..Default::default()
            }),
            ResolvedGitLabCommit::from(GitLabCommit {
                id: Some(String::from("cccccccccccccccccccccccccccccccccccccccc")),
                author_name: Some(String::from("Maintainer Name")),
                author_email: Some(String::from("maintainer@example.com")),
                ..Default::default()
            }),
        ];
        let mrs = vec![
            GitLabMergeRequest {
                iid: Some(15),
                merge_commit_sha: Some(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")),
                author: Some(GitLabUser {
                    name: Some(String::from("Flapjack McButterpants")),
                    username: Some(String::from("flapjack.mcbutterpants_42")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            GitLabMergeRequest {
                iid: Some(42),
                squash_commit_sha: Some(String::from("cccccccccccccccccccccccccccccccccccccccc")),
                author: Some(GitLabUser {
                    name: Some(String::from("External Contributor")),
                    username: Some(String::from("ext_contrib")),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ];

        resolve_from_merge_requests(&mut commits, &mrs);

        assert_eq!(
            Some(String::from("flapjack.mcbutterpants_42")),
            commits[0].author_username
        );
        assert_eq!(
            Some(String::from("flapjack.mcbutterpants_42")),
            commits[1].author_username
        );
        // Maintainer squash/merge commit must keep `author_username` unset rather than
        // overwriting it with the MR author's username.
        assert_eq!(None, commits[2].author_username);
    }

    #[test]
    fn match_user_by_email_requires_exact_email_match() {
        // Single fuzzy match with `public_email: null` must be rejected (no false attribution).
        let unexposed = vec![GitLabUserLookup {
            name: Some(String::from("jane@example.com")),
            username: Some(String::from("spoofed")),
            public_email: None,
            email: None,
        }];
        assert_eq!(None, match_user_by_email("jane@example.com", &unexposed));

        // Prefix/substring mismatch must be rejected.
        let prefix = vec![GitLabUserLookup {
            username: Some(String::from("jdoe")),
            public_email: Some(String::from("jane@example.com.au")),
            ..Default::default()
        }];
        assert_eq!(None, match_user_by_email("jane@example.com", &prefix));

        // Exact case-insensitive `public_email` match succeeds.
        let exact = vec![GitLabUserLookup {
            username: Some(String::from("jdoe")),
            public_email: Some(String::from("Jane@Example.com")),
            ..Default::default()
        }];
        assert_eq!(
            Some(String::from("jdoe")),
            match_user_by_email("jane@example.com", &exact)
        );
    }

    #[test]
    fn resolve_from_project_users_handles_unique_and_ambiguous_names() {
        let mut commits = vec![
            ResolvedGitLabCommit::from(GitLabCommit {
                id: Some(String::from("1111111111111111111111111111111111111111")),
                author_name: Some(String::from("Nathan Belsterling")),
                ..Default::default()
            }),
            ResolvedGitLabCommit::from(GitLabCommit {
                id: Some(String::from("2222222222222222222222222222222222222222")),
                author_name: Some(String::from("John Smith")),
                ..Default::default()
            }),
        ];
        let users = vec![
            GitLabUserLookup {
                name: Some(String::from("Nathan Belsterling")),
                username: Some(String::from("nbelste1")),
                ..Default::default()
            },
            GitLabUserLookup {
                name: Some(String::from("John Smith")),
                username: Some(String::from("jsmith1")),
                ..Default::default()
            },
            GitLabUserLookup {
                name: Some(String::from("John Smith")),
                username: Some(String::from("jsmith2")),
                ..Default::default()
            },
        ];

        resolve_from_project_users(&mut commits, &users);

        assert_eq!(Some(String::from("nbelste1")), commits[0].author_username);
        assert_eq!(None, commits[1].author_username);
    }

    fn spawn_mock_gitlab_server<F>(
        handler: F,
    ) -> (
        String,
        std::sync::Arc<std::sync::Mutex<Vec<String>>>,
        std::sync::Arc<std::sync::atomic::AtomicBool>,
    )
    where
        F: Fn(&str) -> (u16, String) + Send + Sync + 'static,
    {
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpListener;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::{Arc, Mutex};

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
        let addr = listener.local_addr().expect("local addr");
        listener.set_nonblocking(true).expect("set nonblocking");

        let requests = Arc::new(Mutex::new(Vec::new()));
        let requests_clone = Arc::clone(&requests);
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);
        let handler = Arc::new(handler);

        std::thread::spawn(move || {
            while !shutdown_clone.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let handler = Arc::clone(&handler);
                        let requests_clone = Arc::clone(&requests_clone);
                        std::thread::spawn(move || {
                            let _ = stream.set_nonblocking(false);
                            let _ =
                                stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
                            let mut reader =
                                BufReader::new(stream.try_clone().expect("clone tcp stream"));
                            let mut request_line = String::new();
                            if reader.read_line(&mut request_line).is_ok() &&
                                !request_line.is_empty()
                            {
                                loop {
                                    let mut header = String::new();
                                    match reader.read_line(&mut header) {
                                        Ok(0) | Err(_) => break,
                                        Ok(_) if header == "\r\n" || header == "\n" => break,
                                        Ok(_) => {}
                                    }
                                }
                                let path = request_line
                                    .split_whitespace()
                                    .nth(1)
                                    .unwrap_or("/")
                                    .to_string();
                                requests_clone.lock().expect("lock").push(path.clone());
                                let (status, body) = handler(&path);
                                let reason = match status {
                                    200 => "OK",
                                    403 => "Forbidden",
                                    404 => "Not Found",
                                    _ => "Error",
                                };
                                let resp = format!(
                                    "HTTP/1.1 {status} {reason}\r\nContent-Type: \
                                     application/json\r\nCache-Control: \
                                     no-store\r\nContent-Length: {}\r\nConnection: \
                                     close\r\n\r\n{body}",
                                    body.len()
                                );
                                let _ = stream.write_all(resp.as_bytes());
                                let _ = stream.flush();
                            }
                        });
                    }
                    Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                    }
                    Err(_) => break,
                }
            }
        });

        (format!("http://{addr}/api/v4"), requests, shutdown)
    }

    #[tokio::test]
    async fn get_metadata_resolves_commits_with_single_project_users_list_and_release_scoping() {
        use std::sync::atomic::Ordering;

        use crate::commit::Commit;
        use crate::release::Release;

        let (api_url, requests, shutdown) = spawn_mock_gitlab_server(|path| {
            let is_page_1 = path.ends_with("page=1") || path.contains("page=1&");
            if path.starts_with("/api/v4/projects/42/repository/commits") {
                if is_page_1 {
                    (
                        200,
                        concat!(
                            r#"[{"id":"1111111111111111111111111111111111111111","author_name":"Flapjack McButterpants","author_email":"flapjack@example.com","committed_date":"2026-04-25T19:41:39+00:00","parent_ids":[]},"#,
                            r#"{"id":"2222222222222222222222222222222222222222","author_name":"Flapjack","author_email":"flapjack@example.com","committed_date":"2026-04-25T19:42:39+00:00","parent_ids":[]},"#,
                            r#"{"id":"3333333333333333333333333333333333333333","author_name":"Maintainer Name","author_email":"maintainer@example.com","committed_date":"2026-04-25T19:43:39+00:00","parent_ids":[]},"#,
                            r#"{"id":"4444444444444444444444444444444444444444","author_name":"Maintainer Name","author_email":"maintainer@example.com","committed_date":"2026-04-25T19:44:39+00:00","parent_ids":[]},"#,
                            r#"{"id":"5555555555555555555555555555555555555555","author_name":"Nathan Belsterling","author_email":"nbelste1@jhu.edu","committed_date":"2026-04-25T19:45:39+00:00","parent_ids":[]},"#,
                            r#"{"id":"6666666666666666666666666666666666666666","author_name":"Historical Author","author_email":"historical@example.com","committed_date":"2025-01-01T00:00:00+00:00","parent_ids":[]},"#,
                            r#"{"id":"7777777777777777777777777777777777777777","author_name":"Unknown Author","author_email":"unknown@example.com","committed_date":"2026-04-25T19:47:39+00:00","parent_ids":[]}]"#,
                        )
                        .to_string(),
                    )
                } else {
                    (200, "[]".to_string())
                }
            } else if path.starts_with("/api/v4/projects/42/merge_requests") {
                if is_page_1 {
                    (
                        200,
                        concat!(
                            r#"[{"iid":15,"title":"feat: mr","merge_commit_sha":"1111111111111111111111111111111111111111","labels":[],"author":{"name":"Flapjack McButterpants","username":"flapjack.mcbutterpants_42"}},"#,
                            r#"{"iid":16,"title":"feat: squash","squash_commit_sha":"3333333333333333333333333333333333333333","labels":[],"author":{"name":"External Contributor","username":"ext_contributor"}}]"#,
                        )
                        .to_string(),
                    )
                } else {
                    (200, "[]".to_string())
                }
            } else if path.starts_with("/api/v4/users?search=maintainer%40example.com") {
                (
                    200,
                    r#"[{"name":"Maintainer Name","username":"repo_maintainer","public_email":"maintainer@example.com"}]"#
                        .to_string(),
                )
            } else if path.starts_with("/api/v4/users?search=") {
                (200, "[]".to_string())
            } else if path == "/api/v4/projects/42/users?per_page=100&page=1" {
                (
                    200,
                    r#"[{"name":"Nathan Belsterling","username":"nbelste1","public_email":null}]"#
                        .to_string(),
                )
            } else {
                (404, "[]".to_string())
            }
        });

        let remote = Remote {
            owner: String::from("nbelste1"),
            repo: String::from("git-cliff-example"),
            api_url: Some(api_url),
            http_timeout: std::time::Duration::from_secs(10),
            ..Default::default()
        };
        let client = GitLabClient::try_from(remote).expect("build client");
        let relevant_ids = HashSet::from([
            "1111111111111111111111111111111111111111",
            "2222222222222222222222222222222222222222",
            "3333333333333333333333333333333333333333",
            "4444444444444444444444444444444444444444",
            "5555555555555555555555555555555555555555",
            "7777777777777777777777777777777777777777",
        ]);
        let (commits, merge_requests) = client
            .get_metadata(42, None, Some(&relevant_ids))
            .await
            .expect("get_metadata failed");
        shutdown.store(true, Ordering::Relaxed);

        let usernames: HashMap<String, Option<String>> =
            commits.iter().map(|c| (c.id(), c.username())).collect();
        assert_eq!(
            Some(&Some(String::from("flapjack.mcbutterpants_42"))),
            usernames.get("1111111111111111111111111111111111111111")
        );
        assert_eq!(
            Some(&Some(String::from("flapjack.mcbutterpants_42"))),
            usernames.get("2222222222222222222222222222222222222222")
        );
        assert_eq!(
            Some(&Some(String::from("repo_maintainer"))),
            usernames.get("3333333333333333333333333333333333333333")
        );
        assert_eq!(
            Some(&Some(String::from("repo_maintainer"))),
            usernames.get("4444444444444444444444444444444444444444")
        );
        assert_eq!(
            Some(&Some(String::from("nbelste1"))),
            usernames.get("5555555555555555555555555555555555555555")
        );
        assert_eq!(
            Some(&Some(String::from("Unknown Author"))),
            usernames.get("7777777777777777777777777777777777777777")
        );

        // Verify Release::update_gitlab_metadata preserves distinct username vs pr_author
        // and populates gitlab.contributors with resolved handles.
        let mut release = Release {
            commits: vec![
                Commit::from(String::from(
                    "3333333333333333333333333333333333333333 feat: squash",
                )),
                Commit::from(String::from(
                    "5555555555555555555555555555555555555555 docs: direct",
                )),
            ],
            ..Default::default()
        };
        release
            .update_gitlab_metadata(commits, merge_requests)
            .expect("update_gitlab_metadata");
        assert_eq!(
            Some(String::from("repo_maintainer")),
            release.commits[0]
                .remote
                .as_ref()
                .and_then(|r| r.username.clone())
        );
        assert_eq!(
            Some(String::from("ext_contributor")),
            release.commits[0]
                .remote
                .as_ref()
                .and_then(|r| r.pr_author.clone())
        );
        assert_eq!(
            Some(String::from("nbelste1")),
            release.commits[1]
                .remote
                .as_ref()
                .and_then(|r| r.username.clone())
        );
        assert_eq!(
            vec![
                Some(String::from("repo_maintainer")),
                Some(String::from("nbelste1")),
            ],
            release
                .gitlab
                .contributors
                .iter()
                .map(|c| c.username.clone())
                .collect::<Vec<_>>()
        );

        // Verify API hygiene: no lookup for MR-resolved or historical non-release emails,
        // deduplicated maintainer lookup, and a single list GET for `/projects/:id/users`.
        let recorded = requests.lock().expect("lock");
        assert!(
            !recorded
                .iter()
                .any(|p| p.contains("flapjack%40example.com")),
            "MR-resolved author email must not trigger /users lookup: {recorded:?}"
        );
        assert!(
            !recorded
                .iter()
                .any(|p| p.contains("historical%40example.com")),
            "non-release historical commit must not trigger /users lookup: {recorded:?}"
        );
        assert_eq!(
            1,
            recorded
                .iter()
                .filter(|p| p.contains("maintainer%40example.com"))
                .count()
        );
        assert_eq!(
            1,
            recorded
                .iter()
                .filter(|p| p.starts_with("/api/v4/projects/42/users"))
                .count(),
            "project users must be fetched via a single list request when < MAX_PAGE_SIZE: \
             {recorded:?}"
        );
    }

    #[tokio::test]
    async fn get_metadata_short_circuits_forbidden_users_api_and_resolves_project_users() {
        use std::sync::atomic::Ordering;

        let (api_url, requests, shutdown) = spawn_mock_gitlab_server(|path| {
            let is_page_1 = path.ends_with("page=1") || path.contains("page=1&");
            if path.starts_with("/api/v4/projects/99/repository/commits") {
                if is_page_1 {
                    (
                        200,
                        concat!(
                            r#"[{"id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","author_name":"Nathan Belsterling","author_email":"nbelste1@jhu.edu","committed_date":"2026-04-25T19:41:39+00:00","parent_ids":[]},"#,
                            r#"{"id":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","author_name":"Second Author","author_email":"second@example.com","committed_date":"2026-04-25T19:42:39+00:00","parent_ids":[]}]"#,
                        )
                        .to_string(),
                    )
                } else {
                    (200, "[]".to_string())
                }
            } else if path.starts_with("/api/v4/projects/99/merge_requests") {
                (200, "[]".to_string())
            } else if path.starts_with("/api/v4/users?") {
                (403, r#"{"message":"403 Forbidden"}"#.to_string())
            } else if path == "/api/v4/projects/99/users?per_page=100&page=1" {
                (
                    200,
                    r#"[{"name":"Nathan Belsterling","username":"nbelste1","public_email":null}]"#
                        .to_string(),
                )
            } else {
                (404, "[]".to_string())
            }
        });

        let remote = Remote {
            owner: String::from("nbelste1"),
            repo: String::from("git-cliff-example"),
            api_url: Some(api_url),
            http_timeout: std::time::Duration::from_secs(10),
            ..Default::default()
        };
        let client = GitLabClient::try_from(remote).expect("build client");
        let (commits, _) = client
            .get_metadata(99, None, None)
            .await
            .expect("get_metadata should degrade gracefully on 403 /users");
        shutdown.store(true, Ordering::Relaxed);

        let usernames: HashMap<String, Option<String>> =
            commits.iter().map(|c| (c.id(), c.username())).collect();
        assert_eq!(
            Some(&Some(String::from("nbelste1"))),
            usernames.get("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        );
        assert_eq!(
            Some(&Some(String::from("Second Author"))),
            usernames.get("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );

        let recorded = requests.lock().expect("lock");
        assert_eq!(
            1,
            recorded
                .iter()
                .filter(|p| p.starts_with("/api/v4/users?"))
                .count(),
            "should short-circuit /users after 403 Forbidden: {recorded:?}"
        );
        assert_eq!(
            1,
            recorded
                .iter()
                .filter(|p| p.starts_with("/api/v4/projects/99/users"))
                .count()
        );
    }
}
