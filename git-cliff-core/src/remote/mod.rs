/// GitHub client.
#[cfg(feature = "github")]
pub mod github;

/// GitLab client.
#[cfg(feature = "gitlab")]
pub mod gitlab;

/// Gitea client.
#[cfg(feature = "gitea")]
pub mod gitea;

/// Bitbucket client.
#[cfg(feature = "bitbucket")]
pub mod bitbucket;

use std::{
	hash::{
		Hash,
		Hasher,
	},
	time::Duration,
};

use async_trait::async_trait;
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
use reqwest::{
	header::{
		HeaderMap,
		HeaderValue,
	},
	Client,
};
use reqwest_middleware::{
	ClientBuilder,
	ClientWithMiddleware,
};
use secrecy::ExposeSecret;
use serde::{
	de::DeserializeOwned,
	Deserialize,
	Serialize,
};
use url::Url;

use crate::{
	config::{
		Remote,
		RemoteKind,
	},
	error::{
		Error,
		Result,
	},
};

/// User agent for interacting with the remote API.
///
/// This is needed since GitHub API does not accept empty user agent.
pub(crate) const USER_AGENT: &str =
	concat!(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

/// Request timeout value in seconds.
pub(crate) const REQUEST_TIMEOUT: u64 = 30;

/// TCP keeplive value in seconds.
pub(crate) const REQUEST_KEEP_ALIVE: u64 = 60;

/// Maximum number of entries to fetch in a single page.
pub(crate) const MAX_PAGE_SIZE: &str = "100";

/// Commit from a code forge
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RemoteCommit {
	/// Commit SHA.
	pub id:       String,
	/// Commit author.
	pub username: Option<String>,
}

/// Pull request from a code forge
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RemotePullRequest {
	/// Number.
	pub number:       i64,
	/// Title.
	pub title:        Option<String>,
	/// Labels of the pull request.
	pub labels:       Vec<String>,
	/// Merge commit SHA.
	pub merge_commit: Option<String>,
}

/// Result of a remote metadata.
pub type RemoteMetadata = (Vec<RemoteCommit>, Vec<RemotePullRequest>);

/// Metadata of a remote release.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RemoteReleaseMetadata {
	/// Contributors.
	pub contributors: Vec<RemoteContributor>,
}

/// Representation of a remote contributor.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RemoteContributor {
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

impl Hash for RemoteContributor {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.username.hash(state);
	}
}

/// Trait for handling the different entries returned from the remote.
trait RemoteEntry {
	/// Returns the API URL for fetching the entries at the specified page.
	fn url(project_id: i64, api_url: &Url, remote: &Remote, page: i32) -> Url;
	/// Returns the request buffer size.
	fn buffer_size() -> usize;
	/// Whether the client should exit early after fetching this entry (e.g. no
	/// items any more).
	fn early_exit(&self) -> bool {
		false
	}
}

/// Client for fetching code forge data
#[async_trait]
pub trait RemoteClient: Send {
	/// Initializes the API client and fetches project ID if necessary.
	async fn init(&mut self) -> Result<()> {
		Ok(())
	}

	/// Fetches the API and returns the commits.
	async fn get_commits(&self) -> Result<Vec<RemoteCommit>>;

	/// Fetches the API and returns the pull requests.
	async fn get_pull_requests(&self) -> Result<Vec<RemotePullRequest>>;
}

trait RemoteClientInternal {
	/// Returns the API url.
	fn api_url(&self) -> &Url;

	/// Returns the remote repository information.
	fn remote(&self) -> Remote;

	/// Returns the HTTP client for making requests.
	fn client(&self) -> ClientWithMiddleware;

	/// Returns true if the client should early exit.
	fn early_exit<T: DeserializeOwned + RemoteEntry>(&self, page: &T) -> bool {
		page.early_exit()
	}

	/// Retrieves a single object.
	async fn get_entry<T: DeserializeOwned + RemoteEntry>(
		&self,
		project_id: i64,
		page: i32,
	) -> Result<T> {
		let url = T::url(project_id, self.api_url(), &self.remote(), page);
		debug!("Sending request to: {url}");
		let response = self.client().get(url).send().await?;
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

	/// Retrieves a single page of entries.
	async fn get_entries_with_page<T: DeserializeOwned + RemoteEntry>(
		&self,
		project_id: i64,
		page: i32,
	) -> Result<Vec<T>> {
		let url = T::url(project_id, self.api_url(), &self.remote(), page);
		debug!("Sending request to: {url}");
		let response = self.client().get(url).send().await?;
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

	/// Fetches the remote API and returns the given entry.
	///
	/// See `fetch_with_early_exit` for the early exit version of this method.
	async fn fetch<T: DeserializeOwned + RemoteEntry>(
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

	/// Fetches the remote API and returns the given entry.
	///
	/// Early exits based on the response.
	async fn fetch_with_early_exit<T: DeserializeOwned + RemoteEntry>(
		&self,
		project_id: i64,
	) -> Result<Vec<T>> {
		let entries: Vec<T> = stream::iter(0..)
			.map(|i| self.get_entry::<T>(project_id, i))
			.buffered(T::buffer_size())
			.take_while(|page| {
				let status = match page {
					Ok(v) => !self.early_exit(v),
					Err(e) => {
						debug!("Error while fetching page: {:?}", e);
						true
					}
				};
				future::ready(status && page.is_ok())
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
		Ok(entries)
	}
}

/// Creates a HTTP client for the remote.
fn create_remote_client(
	remote: &Remote,
	accept_header: &str,
) -> Result<ClientWithMiddleware> {
	if !remote.is_set() {
		return Err(Error::RemoteNotSetError);
	}
	let mut headers = HeaderMap::new();
	headers.insert(
		reqwest::header::ACCEPT,
		HeaderValue::from_str(accept_header)?,
	);
	if let Some(token) = &remote.token {
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
	Ok(client)
}

struct ApiUrlCfg {
	/// Default API URL
	api_url:        &'static str,
	/// Environment variable for overriding the API URL
	env_var:        &'static str,
	/// Path to add to the base URL from the remote config to get the API URL
	api_path:       &'static [&'static str],
	/// Use the default `api_url` if the domain from the remote URL matches this
	/// value. This can be used if the hosted instance uses a different API URL
	/// format than self-hosted instances.
	///
	/// Setting this value to an empty string disables this feature
	default_domain: &'static str,
}

impl ApiUrlCfg {
	/// Get the API URL to be used for accessing the given Remote. The URL is
	/// parsed from the following sources:
	///
	/// 1. Environment variable (`<FORGE>_API_URL`)
	/// 2. Configured URL from the Remote
	/// 3. Default api_url
	fn get_api_url(&self, remote: &Remote) -> Result<Url> {
		if let Ok(url) = std::env::var(self.env_var) {
			Ok(Url::parse(&url)?)
		} else if let Some(cfg_url) = remote.url.as_ref().filter(|url| {
			self.default_domain.is_empty() ||
				url.domain() != Some(self.default_domain)
		}) {
			let mut url = cfg_url.clone();
			url.path_segments_mut()
				.expect("invalid url")
				.extend(self.api_path);
			Ok(url)
		} else {
			Ok(Url::parse(self.api_url)?)
		}
	}
}

/// Create a new remote client from the give kind and configuration
pub fn new_remote_client(
	kind: RemoteKind,
	remote: Remote,
) -> Result<Box<dyn RemoteClient>> {
	#[cfg(feature = "github")]
	use github::GitHubClient;

	#[cfg(feature = "gitlab")]
	use gitlab::GitLabClient;

	#[cfg(feature = "gitea")]
	use gitea::GiteaClient;

	#[cfg(feature = "bitbucket")]
	use bitbucket::BitbucketClient;

	#[allow(unreachable_patterns)]
	Ok(match kind {
		#[cfg(feature = "github")]
		RemoteKind::GitHub => Box::new(GitHubClient::try_from(remote)?),
		#[cfg(feature = "gitlab")]
		RemoteKind::GitLab => Box::new(GitLabClient::try_from(remote)?),
		#[cfg(feature = "gitea")]
		RemoteKind::Gitea => Box::new(GiteaClient::try_from(remote)?),
		#[cfg(feature = "bitbucket")]
		RemoteKind::Bitbucket => Box::new(BitbucketClient::try_from(remote)?),
		_ => panic!(
			"{} client is not enabled. Please build git-cliff with the {} feature \
			 enabled",
			kind,
			kind.id()
		),
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_api_url() {
		let remote = Remote {
			url:   Some(Url::parse("https://github.com").expect("invalid URL")),
			owner: "orhun".to_owned(),
			repo:  "git-cliff".to_owned(),
			token: None,
		};

		let cfg = ApiUrlCfg {
			api_url:        "https://api.github.com",
			env_var:        "GITHUB_API_URL",
			api_path:       &["api", "v3"],
			default_domain: "github.com",
		};

		let res = cfg.get_api_url(&remote).expect("could not get api url");
		assert_eq!("https://api.github.com/", res.to_string());
	}

	#[test]
	fn get_api_url_subpath() {
		let remote = Remote {
			url:   Some(
				Url::parse("https://example.com/gitea").expect("invalid URL"),
			),
			owner: "orhun".to_owned(),
			repo:  "git-cliff".to_owned(),
			token: None,
		};

		let cfg = ApiUrlCfg {
			api_url:        "https://codeberg.org/api/v1",
			env_var:        "GITEA_API_URL",
			api_path:       &["api", "v1"],
			default_domain: "",
		};

		let res = cfg.get_api_url(&remote).expect("could not get api url");
		assert_eq!("https://example.com/gitea/api/v1", res.to_string());
	}
}
