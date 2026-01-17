use std::env;
use std::fs;
use std::path::PathBuf;

use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use url::Url;

use super::*;
use crate::config::Remote;
use crate::error::*;

/// Log message to show while fetching data from GitLab.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitLab...";

/// Log message to show when done fetching from GitLab.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitLab data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["gitlab", "commit.gitlab", "commit.remote"];

/// Resolve GitLab API token with priority `GITLAB_TOKEN` -> netrc -> config.
pub fn gitlab_token(remote: &Remote) -> Result<Option<SecretString>> {
    if let Ok(token) = env::var("GITLAB_TOKEN") {
        return Ok(Some(SecretString::new(token)));
    }

    if let Some(host) = gitlab_host(remote) {
        if let Some(token) = token_from_netrc(&host)? {
            return Ok(Some(token));
        }
    }

    Ok(remote.token.clone())
}

fn gitlab_host(remote: &Remote) -> Option<String> {
    let api_url = env::var(GitLabClient::API_URL_ENV)
        .ok()
        .or_else(|| remote.api_url.clone())
        .unwrap_or_else(|| GitLabClient::API_URL.to_string());
    parse_host(&api_url)
}

fn parse_host(api_url: &str) -> Option<String> {
    if let Ok(url) = Url::parse(api_url) {
        if let Some(host) = url.host_str() {
            return Some(host.to_string());
        }
    }

    let trimmed = api_url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let host = trimmed.split('/').next().unwrap_or("").trim();
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn netrc_paths() -> Vec<PathBuf> {
    if let Ok(custom) = env::var("NETRC") {
        return vec![PathBuf::from(custom)];
    }

    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".netrc"));
        #[cfg(windows)]
        paths.push(home.join("_netrc"));
    }
    paths
}

fn token_from_netrc(host: &str) -> Result<Option<SecretString>> {
    for path in netrc_paths() {
        match fs::read_to_string(&path) {
            Ok(contents) => {
                if let Some(token) = parse_netrc(&contents, host) {
                    return Ok(Some(SecretString::new(token)));
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => {
                log::debug!("Failed to read netrc file {}: {}", path.display(), err);
                continue;
            }
        }
    }
    Ok(None)
}

fn parse_netrc(contents: &str, host: &str) -> Option<String> {
    let tokens: Vec<String> = contents
        .lines()
        .map(|line| line.split('#').next().unwrap_or("").trim())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_whitespace().map(str::to_string))
        .collect();

    let mut i = 0;
    let mut default_password: Option<String> = None;

    while i < tokens.len() {
        if tokens[i] == "machine" && i + 1 < tokens.len() {
            let machine = tokens[i + 1].clone();
            i += 2;

            let mut password: Option<String> = None;

            while i < tokens.len() {
                match tokens[i].as_str() {
                    "machine" => break,
                    "password" if i + 1 < tokens.len() => {
                        password = Some(tokens[i + 1].clone());
                        i += 2;
                    }
                    _ => i += 1,
                }
            }

            if machine == "default" {
                default_password = password.clone().or(default_password);
            }

            if machine == host {
                return password.or(default_password);
            }
        } else {
            i += 1;
        }
    }

    default_password
}

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
            "{}/projects/{}/repository/commits?per_page={MAX_PAGE_SIZE}&page={page}",
            api_url, project_id
        );

        if let Some(ref_name) = ref_name {
            url.push_str(&format!("&ref_name={}", ref_name));
        }

        url
    }
    /// Constructs the URL for GitLab merge requests API.
    fn pull_requests_url(project_id: i64, api_url: &str, page: i32) -> String {
        format!(
            "{}/projects/{}/merge_requests?per_page={MAX_PAGE_SIZE}&page={page}&state=merged",
            api_url, project_id
        )
    }

    /// Looks up the project details.
    pub async fn get_project(&self) -> Result<GitLabProject> {
        let url = Self::project_url(&self.api_url(), &self.remote());
        self.get_json::<GitLabProject>(&url).await
    }

    /// Fetches the complete list of commits.
    /// This is inefficient for large repositories; consider using
    /// `get_commit_stream` instead.
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
    pub async fn get_pull_requests(
        &self,
        project_id: i64,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        use futures::TryStreamExt;
        self.get_pull_request_stream(project_id).try_collect().await
    }

    fn get_commit_stream<'a>(
        &'a self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> impl Stream<Item = Result<Box<dyn RemoteCommit>>> + 'a {
        let ref_name = ref_name.map(|s| s.to_string());
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

    fn get_pull_request_stream<'a>(
        &'a self,
        project_id: i64,
    ) -> impl Stream<Item = Result<Box<dyn RemotePullRequest>>> + 'a {
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
    use std::env;
    use std::fs;
    use std::sync::Mutex;

    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use secrecy::{ExposeSecret, SecretString};
    use temp_dir::TempDir;

    use super::*;

    lazy_static! {
        static ref ENV_LOCK: Mutex<()> = Mutex::new(());
    }

    fn set_var(key: &str, value: &str) {
        // Env var mutation is used for test isolation; safe in this single-threaded guard.
        unsafe { env::set_var(key, value) }
    }

    fn remove_var(key: &str) {
        unsafe { env::remove_var(key) }
    }

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
            id: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
            author_name: String::from("orhun"),
            committed_date: String::from("2021-07-18T15:14:39+03:00"),
            ..Default::default()
        };

        assert_eq!(Some(1626610479), remote_commit.timestamp());
    }

    #[test]
    fn pull_request_no_merge_commit() {
        let mr = GitLabMergeRequest {
            sha: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
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
    fn gitlab_token_prefers_env_over_netrc_and_config() {
        let _guard = ENV_LOCK.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let netrc_path = temp_dir.path().join("netrc");
        fs::write(
            &netrc_path,
            "machine gitlab.com login oauth2 password netrc-token",
        )
        .unwrap();

        let previous_netrc = env::var("NETRC").ok();
        let previous_env = env::var("GITLAB_TOKEN").ok();
        set_var("NETRC", netrc_path.to_str().unwrap());
        set_var("GITLAB_TOKEN", "env-token");

        let mut remote = Remote::new("owner", "repo");
        remote.api_url = Some("https://gitlab.com/api/v4".into());
        remote.token = Some(SecretString::new("config-token".into()));

        let token = gitlab_token(&remote).unwrap().unwrap();
        assert_eq!("env-token", token.expose_secret());

        match previous_netrc {
            Some(value) => set_var("NETRC", &value),
            None => remove_var("NETRC"),
        }
        match previous_env {
            Some(value) => set_var("GITLAB_TOKEN", &value),
            None => remove_var("GITLAB_TOKEN"),
        }
    }

    #[test]
    fn gitlab_token_prefers_netrc_over_config() {
        let _guard = ENV_LOCK.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let netrc_path = temp_dir.path().join("netrc");
        fs::write(
            &netrc_path,
            "machine gitlab.com login gitlab-ci-token password netrc-token",
        )
        .unwrap();

        let previous_netrc = env::var("NETRC").ok();
        let previous_env = env::var("GITLAB_TOKEN").ok();
        set_var("NETRC", netrc_path.to_str().unwrap());
        remove_var("GITLAB_TOKEN");

        let mut remote = Remote::new("owner", "repo");
        remote.api_url = Some("https://gitlab.com/api/v4".into());
        remote.token = Some(SecretString::new("config-token".into()));

        let token = gitlab_token(&remote).unwrap().unwrap();
        assert_eq!("netrc-token", token.expose_secret());

        match previous_netrc {
            Some(value) => set_var("NETRC", &value),
            None => remove_var("NETRC"),
        }
        match previous_env {
            Some(value) => set_var("GITLAB_TOKEN", &value),
            None => remove_var("GITLAB_TOKEN"),
        }
    }

    #[test]
    fn gitlab_token_uses_custom_api_host() {
        let _guard = ENV_LOCK.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let netrc_path = temp_dir.path().join("netrc");
        fs::write(
            &netrc_path,
            "machine gitlab.example.com login oauth2 password scoped-token",
        )
        .unwrap();

        let previous_netrc = env::var("NETRC").ok();
        set_var("NETRC", netrc_path.to_str().unwrap());

        let mut remote = Remote::new("owner", "repo");
        remote.api_url = Some("https://gitlab.example.com/api/v4/".into());

        let token = gitlab_token(&remote).unwrap().unwrap();
        assert_eq!("scoped-token", token.expose_secret());

        match previous_netrc {
            Some(value) => set_var("NETRC", &value),
            None => remove_var("NETRC"),
        }
    }

    #[test]
    fn gitlab_token_falls_back_to_config() {
        let _guard = ENV_LOCK.lock().unwrap();
        let previous_netrc = env::var("NETRC").ok();
        let previous_env = env::var("GITLAB_TOKEN").ok();
        remove_var("NETRC");
        remove_var("GITLAB_TOKEN");

        let mut remote = Remote::new("owner", "repo");
        remote.token = Some(SecretString::new("config-token".into()));

        let token = gitlab_token(&remote).unwrap().unwrap();
        assert_eq!("config-token", token.expose_secret());

        match previous_netrc {
            Some(value) => set_var("NETRC", &value),
            None => remove_var("NETRC"),
        }
        match previous_env {
            Some(value) => set_var("GITLAB_TOKEN", &value),
            None => remove_var("GITLAB_TOKEN"),
        }
    }

    #[test]
    fn gitlab_token_uses_default_machine_when_host_missing() {
        let _guard = ENV_LOCK.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let netrc_path = temp_dir.path().join("netrc");
        fs::write(
            &netrc_path,
            "machine default login oauth2 password default-token",
        )
        .unwrap();

        let previous_netrc = env::var("NETRC").ok();
        let previous_env = env::var("GITLAB_TOKEN").ok();
        set_var("NETRC", netrc_path.to_str().unwrap());
        remove_var("GITLAB_TOKEN");

        let remote = Remote::new("owner", "repo");
        let token = gitlab_token(&remote).unwrap().unwrap();
        assert_eq!("default-token", token.expose_secret());

        match previous_netrc {
            Some(value) => set_var("NETRC", &value),
            None => remove_var("NETRC"),
        }
        match previous_env {
            Some(value) => set_var("GITLAB_TOKEN", &value),
            None => remove_var("GITLAB_TOKEN"),
        }
    }
}
