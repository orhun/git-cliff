/// GitHub client.
#[cfg(feature = "github")]
pub mod github;

/// GitLab client.
#[cfg(feature = "gitlab")]
pub mod gitlab;

use serde::{
	Deserialize,
	Serialize,
};
use std::hash::{
	Hash,
	Hasher,
};

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
pub(crate) trait RemoteEntry {
	/// Returns the API URL for fetching the entries at the specified page.
	fn url(project_id: i64, repo: &str, owner: &str, page: i32) -> String;
	/// Returns the request buffer size.
	fn buffer_size() -> usize;
}
