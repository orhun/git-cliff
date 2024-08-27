use serde::{
	Deserialize,
	Serialize,
};
use std::hash::{
	Hash,
	Hasher,
};

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
