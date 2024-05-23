use crate::config::Remote;
use crate::error::{
	Error,
	Result,
};
use futures::{
	future,
	stream,
	StreamExt,
};
use http_cache_reqwest::{
	CACacheManager,
	Cache,
	CacheMode,
	HttpCache,
	HttpCacheOptions,
};
use reqwest::header::{
	HeaderMap,
	HeaderValue,
};
use reqwest::Client;
use reqwest_middleware::{
	ClientBuilder,
	ClientWithMiddleware,
};
use secrecy::ExposeSecret;
use serde::de::DeserializeOwned;
use serde::{
	Deserialize,
	Serialize,
};
use std::env;
use std::hash::{
	Hash,
	Hasher,
};
use std::time::Duration;

/// GitLab REST API url.
const GITLAB_API_URL: &str = "https://gitlab.com/api/v4";

/// Environment variable for overriding the GitLab REST API url.
const GITLAB_API_URL_ENV: &str = "GITLAB_API_URL";

/// User agent for interacting with the GitLab API.
///
/// This is needed since GitLab API does not accept empty user agent.
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

/// Request timeout value in seconds.
const REQUEST_TIMEOUT: u64 = 30;

/// TCP keeplive value in seconds.
const REQUEST_KEEP_ALIVE: u64 = 60;

/// Maximum number of entries to fetch in a single page.
const MAX_PAGE_SIZE: usize = 100;

/// Log message to show while fetching data from GitLab.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitLab...";

/// Log message to show when done fetching from GitLab.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitLab data.";

/// Returns the GitLab API url either from environment or from default value.
fn get_gitlab_api_url() -> String {
	env::var(GITLAB_API_URL_ENV)
		.ok()
		.unwrap_or_else(|| GITLAB_API_URL.to_string())
}

/// Trait for handling the different entries returned from the GitLab API.
trait GitLabEntry {
	/// Returns the API URL for fetching the entries at the specified page.
	fn url(id: i64, repo: &str, owner: &str, page: i32) -> String;
	/// Returns the request buffer size.
	fn buffer_size() -> usize;
}

/// https://docs.gitlab.com/ee/api/projects.html#get-single-project
/// Representation of a single GitLab Project.
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

impl GitLabEntry for GitLabProject {
	fn url(_id: i64, repo: &str, owner: &str, _page: i32) -> String {
		format!("{}/projects/{}%2F{}", get_gitlab_api_url(), owner, repo)
	}
	fn buffer_size() -> usize {
		1
	}
}

/// https://docs.gitlab.com/ee/api/commits.html
/// Representation of a single commit.
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

impl GitLabEntry for GitLabCommit {
	fn url(id: i64, _repo: &str, _owner: &str, page: i32) -> String {
		let commit_page = page + 1;
		format!(
			"{}/projects/{}/repository/commits?per_page={MAX_PAGE_SIZE}&\
			 page={commit_page}",
			get_gitlab_api_url(),
			id
		)
	}
	fn buffer_size() -> usize {
		10
	}
}

/// https://docs.gitlab.com/ee/api/merge_requests.html
/// Representation of a single pull request.
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

impl GitLabEntry for GitLabMergeRequest {
	fn url(id: i64, _repo: &str, _owner: &str, page: i32) -> String {
		format!(
			"{}/projects/{}/merge_requests?per_page={MAX_PAGE_SIZE}&page={page}&\
			 state=merged",
			get_gitlab_api_url(),
			id
		)
	}

	fn buffer_size() -> usize {
		5
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

/// Metadata of a GitLab release.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabReleaseMetadata {
	/// Contributors.
	pub contributors: Vec<GitLabContributor>,
}

/// Representation of a GitLab contributor.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitLabContributor {
	/// Username.
	pub username:      Option<String>,
	/// Title of the pull request.
	pub pr_title:      Option<String>,
	/// The pull request that the user created.
	pub pr_number:     Option<i64>,
	/// Labels of the pull request.
	pub pr_labels:     Vec<String>,
	/// Whether if the user contributed for the first time.
	pub is_first_time: bool,
}

impl Hash for GitLabContributor {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.username.hash(state);
	}
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
	/// Owner of the repository.
	owner:  String,
	/// GitLab repository.
	repo:   String,
	/// HTTP client.
	client: ClientWithMiddleware,
}

/// Constructs a GitLab client from the remote configuration.
impl TryFrom<Remote> for GitLabClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		if !remote.is_set() {
			return Err(Error::RemoteNotSetError);
		}
		let mut headers = HeaderMap::new();
		headers.insert(
			reqwest::header::ACCEPT,
			HeaderValue::from_static("application/json"),
		);
		if let Some(token) = remote.token {
			headers.insert(
				reqwest::header::AUTHORIZATION,
				format!("Bearer {}", token.expose_secret()).parse()?,
			);
		}
		headers.insert(reqwest::header::USER_AGENT, USER_AGENT.parse()?);
		let client = Client::builder()
			.timeout(Duration::from_secs(REQUEST_TIMEOUT))
			.tcp_keepalive(Duration::from_secs(REQUEST_KEEP_ALIVE))
			.default_headers(headers)
			.build()?;
		let client = ClientBuilder::new(client)
			.with(Cache(HttpCache {
				mode:    CacheMode::Default,
				manager: CACacheManager {
					path: dirs::cache_dir()
						.ok_or_else(|| {
							Error::DirsError(String::from(
								"failed to find the user's cache directory",
							))
						})?
						.join(env!("CARGO_PKG_NAME")),
				},
				options: HttpCacheOptions::default(),
			}))
			.build();
		Ok(Self {
			owner: remote.owner,
			repo: remote.repo,
			client,
		})
	}
}

impl GitLabClient {
	/// Retrieves a single page of entries.
	async fn get_entries_with_page<T: DeserializeOwned + GitLabEntry>(
		&self,
		project_id: i64,
		page: i32,
	) -> Result<Vec<T>> {
		let url = T::url(project_id, &self.repo, &self.owner, page);
		debug!("Sending request to: {url}");
		let response = self.client.get(&url).send().await?;
		let response_text = if response.status().is_success() {
			let text = response.text().await?;
			trace!("Response: {:?}", text);
			text
		} else {
			let text = response.text().await?;
			error!("Request error: {}", text);
			text
		};
		let response = serde_json::from_str::<Vec<T>>(&response_text)?;
		if response.is_empty() {
			Err(Error::PaginationError(String::from("end of entries")))
		} else {
			Ok(response)
		}
	}

	/// Fetches the GitLab API returns the given entry.
	async fn fetch<T: DeserializeOwned + GitLabEntry>(
		&self,
		project_id: i64,
	) -> Result<Vec<T>> {
		let entries: Vec<Vec<T>> = stream::iter(0..)
			.map(|i| self.get_entries_with_page(project_id, i))
			.buffered(T::buffer_size())
			.take_while(|page| {
				if let Err(e) = page {
					debug!("Error while fetching page: {:?}", e);
				}
				future::ready(page.is_ok())
			})
			.map(|page| match page {
				Ok(v) => v,
				Err(ref e) => {
					log::error!("{:#?}", e);
					page.expect("failed to fetch page: {}")
				}
			})
			.collect()
			.await;
		Ok(entries.into_iter().flatten().collect())
	}

	/// Retrieves a single object.
	async fn get_entry<T: DeserializeOwned + GitLabEntry>(&self) -> Result<T> {
		let url = T::url(0, &self.repo, &self.owner, 1);
		debug!("Sending request to: {url}");
		let response = self.client.get(&url).send().await?;
		let response_text = if response.status().is_success() {
			let text = response.text().await?;
			trace!("Response: {:?}", text);
			text
		} else {
			let text = response.text().await?;
			error!("Request error: {}", text);
			text
		};
		Ok(serde_json::from_str::<T>(&response_text)?)
	}

	/// Fetches the GitLab API and returns the pull requests.
	pub async fn get_project(&self) -> Result<GitLabProject> {
		self.get_entry::<GitLabProject>().await
	}

	/// Fetches the GitLab API and returns the commits.
	pub async fn get_commits(&self, project_id: i64) -> Result<Vec<GitLabCommit>> {
		self.fetch::<GitLabCommit>(project_id).await
	}

	/// Fetches the GitLab API and returns the pull requests.
	pub async fn get_merge_requests(
		&self,
		project_id: i64,
	) -> Result<Vec<GitLabMergeRequest>> {
		self.fetch::<GitLabMergeRequest>(project_id).await
	}
}
