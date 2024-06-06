/// GitHub client.
#[cfg(feature = "github")]
pub mod github;

/// GitLab client.
#[cfg(feature = "gitlab")]
pub mod gitlab;

/// Bitbucket client.
#[cfg(feature = "bitbucket")]
pub mod bitbucket;

/// Gitea client.
#[cfg(feature = "gitea")]
pub mod gitea;

use crate::config::Remote;
use crate::error::{
	Error,
	Result,
};
use dyn_clone::DynClone;
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
use std::fmt::Debug;
use std::hash::{
	Hash,
	Hasher,
};
use std::time::Duration;

/// User agent for interacting with the GitHub API.
///
/// This is needed since GitHub API does not accept empty user agent.
pub(crate) const USER_AGENT: &str =
	concat!(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

/// Request timeout value in seconds.
pub(crate) const REQUEST_TIMEOUT: u64 = 30;

/// TCP keeplive value in seconds.
pub(crate) const REQUEST_KEEP_ALIVE: u64 = 60;

/// Maximum number of entries to fetch in a single page.
pub(crate) const MAX_PAGE_SIZE: usize = 100;

/// Trait for handling the different entries returned from the remote.
pub trait RemoteEntry {
	/// Returns the API URL for fetching the entries at the specified page.
	fn url(project_id: i64, api_url: &str, remote: &Remote, page: i32) -> String;
	/// Returns the request buffer size.
	fn buffer_size() -> usize;
	/// Whether if exit early.
	fn early_exit(&self) -> bool;
}

/// Trait for handling remote commits.
pub trait RemoteCommit: DynClone {
	/// Commit SHA.
	fn id(&self) -> String;
	/// Commit author.
	fn username(&self) -> Option<String>;
}

dyn_clone::clone_trait_object!(RemoteCommit);

/// Trait for handling remote pull requests.
pub trait RemotePullRequest: DynClone {
	/// Number.
	fn number(&self) -> i64;
	/// Title.
	fn title(&self) -> Option<String>;
	/// Labels of the pull request.
	fn labels(&self) -> Vec<String>;
	/// Merge commit SHA.
	fn merge_commit(&self) -> Option<String>;
}

dyn_clone::clone_trait_object!(RemotePullRequest);

/// Result of a remote metadata.
pub type RemoteMetadata =
	(Vec<Box<dyn RemoteCommit>>, Vec<Box<dyn RemotePullRequest>>);

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

/// Trait for handling the API connection and fetching.
pub trait RemoteClient {
	/// Returns the API url.
	fn api_url() -> String;

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
		let url = T::url(project_id, &Self::api_url(), &self.remote(), page);
		debug!("Sending request to: {url}");
		let response = self.client().get(&url).send().await?;
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
		let url = T::url(project_id, &Self::api_url(), &self.remote(), page);
		debug!("Sending request to: {url}");
		let response = self.client().get(&url).send().await?;
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

/// Generates a function for updating the release metadata for a remote.
#[doc(hidden)]
#[macro_export]
macro_rules! update_release_metadata {
	($remote: ident, $fn: ident) => {
		impl<'a> Release<'a> {
			/// Updates the remote metadata that is contained in the release.
			///
			/// This function takes two arguments:
			///
			/// - Commits: needed for associating the Git user with the GitHub
			///   username.
			/// - Pull requests: needed for generating the contributor list for the
			///   release.
			pub fn $fn(
				&mut self,
				mut commits: Vec<Box<dyn RemoteCommit>>,
				pull_requests: Vec<Box<dyn RemotePullRequest>>,
			) -> Result<()> {
				let mut contributors: Vec<RemoteContributor> = Vec::new();
				// retain the commits that are not a part of this release for later
				// on checking the first contributors.
				commits.retain(|v| {
					if let Some(commit) =
						self.commits.iter_mut().find(|commit| commit.id == v.id())
					{
						let pull_request = pull_requests
							.iter()
							.find(|pr| pr.merge_commit() == Some(v.id().clone()));
						commit.$remote.username = v.username();
						commit.$remote.pr_number = pull_request.map(|v| v.number());
						commit.$remote.pr_title =
							pull_request.and_then(|v| v.title().clone());
						commit.$remote.pr_labels = pull_request
							.map(|v| v.labels().clone())
							.unwrap_or_default();
						if !contributors
							.iter()
							.any(|v| commit.$remote.username == v.username)
						{
							contributors.push(RemoteContributor {
								username:      commit.$remote.username.clone(),
								pr_title:      commit.$remote.pr_title.clone(),
								pr_number:     commit.$remote.pr_number,
								pr_labels:     commit.$remote.pr_labels.clone(),
								is_first_time: false,
							});
						}
						false
					} else {
						true
					}
				});
				// mark contributors as first-time
				self.$remote.contributors = contributors
					.into_iter()
					.map(|mut v| {
						v.is_first_time = !commits
							.iter()
							.map(|v| v.username())
							.any(|login| login == v.username);
						v
					})
					.collect();
				Ok(())
			}
		}
	};
}
