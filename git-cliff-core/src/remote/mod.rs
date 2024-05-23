/// GitHub client.
#[cfg(feature = "github")]
pub mod github;

/// GitLab client.
#[cfg(feature = "gitlab")]
pub mod gitlab;

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
pub(crate) trait RemoteEntry {
	/// Returns the API URL for fetching the entries at the specified page.
	fn url(project_id: i64, repo: &str, owner: &str, page: i32) -> String;
	/// Returns the request buffer size.
	fn buffer_size() -> usize;
}
