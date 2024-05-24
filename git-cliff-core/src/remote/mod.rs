/// GitHub client.
#[cfg(feature = "github")]
pub mod github;

/// GitLab client.
#[cfg(feature = "gitlab")]
pub mod gitlab;

use dyn_clone::DynClone;
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

/// Generates a function for updating the release metadata for a remote.
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
