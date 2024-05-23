use crate::config::Remote;
use crate::error::*;
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

/// GitHub REST API url.
const GITHUB_API_URL: &str = "https://api.github.com";

/// Environment variable for overriding the GitHub REST API url.
const GITHUB_API_URL_ENV: &str = "GITHUB_API_URL";

/// User agent for interacting with the GitHub API.
///
/// This is needed since GitHub API does not accept empty user agent.
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

/// Request timeout value in seconds.
const REQUEST_TIMEOUT: u64 = 30;

/// TCP keeplive value in seconds.
const REQUEST_KEEP_ALIVE: u64 = 60;

/// Maximum number of entries to fetch in a single page.
const MAX_PAGE_SIZE: usize = 100;

/// Log message to show while fetching data from GitHub.
pub const START_FETCHING_MSG: &str = "Retrieving data from GitHub...";

/// Log message to show when done fetching from GitHub.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching GitHub data.";

/// Returns the GitHub API url either from environment or from default value.
fn get_github_api_url() -> String {
	env::var(GITHUB_API_URL_ENV)
		.ok()
		.unwrap_or_else(|| GITHUB_API_URL.to_string())
}

/// Trait for handling the different entries returned from the GitHub API.
trait GitHubEntry {
	/// Returns the API URL for fetching the entries at the specified page.
	fn url(owner: &str, repo: &str, page: i32) -> String;
	/// Returns the request buffer size.
	fn buffer_size() -> usize;
}

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommit {
	/// SHA.
	pub sha:    String,
	/// Author of the commit.
	pub author: Option<GitHubCommitAuthor>,
}

impl GitHubEntry for GitHubCommit {
	fn url(owner: &str, repo: &str, page: i32) -> String {
		format!(
			"{}/repos/{}/{}/commits?per_page={MAX_PAGE_SIZE}&page={page}",
			get_github_api_url(),
			owner,
			repo
		)
	}
	fn buffer_size() -> usize {
		10
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

impl GitHubEntry for GitHubPullRequest {
	fn url(owner: &str, repo: &str, page: i32) -> String {
		format!(
			"{}/repos/{}/{}/pulls?per_page={MAX_PAGE_SIZE}&page={page}&state=closed",
			get_github_api_url(),
			owner,
			repo
		)
	}

	fn buffer_size() -> usize {
		5
	}
}

/// Metadata of a GitHub release.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitHubReleaseMetadata {
	/// Contributors.
	pub contributors: Vec<GitHubContributor>,
}

/// Representation of a GitHub contributor.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct GitHubContributor {
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

impl Hash for GitHubContributor {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.username.hash(state);
	}
}

/// HTTP client for handling GitHub REST API requests.
#[derive(Debug, Clone)]
pub struct GitHubClient {
	/// Owner of the repository.
	owner:  String,
	/// GitHub repository.
	repo:   String,
	/// HTTP client.
	client: ClientWithMiddleware,
}

/// Constructs a GitHub client from the remote configuration.
impl TryFrom<Remote> for GitHubClient {
	type Error = Error;
	fn try_from(remote: Remote) -> Result<Self> {
		if !remote.is_set() {
			return Err(Error::RemoteNotSetError);
		}
		let mut headers = HeaderMap::new();
		headers.insert(
			reqwest::header::ACCEPT,
			HeaderValue::from_static("application/vnd.github+json"),
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

impl GitHubClient {
	/// Retrieves a single page of entries.
	async fn get_entries_with_page<T: DeserializeOwned + GitHubEntry>(
		&self,
		page: i32,
	) -> Result<Vec<T>> {
		let url = T::url(&self.owner, &self.repo, page);
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

	/// Fetches the GitHub API returns the given entry.
	async fn fetch<T: DeserializeOwned + GitHubEntry>(&self) -> Result<Vec<T>> {
		let entries: Vec<Vec<T>> = stream::iter(1..)
			.map(|i| self.get_entries_with_page(i))
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

	/// Fetches the GitHub API and returns the commits.
	pub async fn get_commits(&self) -> Result<Vec<GitHubCommit>> {
		self.fetch::<GitHubCommit>().await
	}

	/// Fetches the GitHub API and returns the pull requests.
	pub async fn get_pull_requests(&self) -> Result<Vec<GitHubPullRequest>> {
		self.fetch::<GitHubPullRequest>().await
	}
}
