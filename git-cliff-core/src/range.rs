use serde::{
	Deserialize,
	Serialize,
};

use crate::commit::Commit;

/// Commit range (from..to or from_short..to_short)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
	/// Full commit SHA the range starts at
	from:       String,
	/// Full commit SHA the range ends at
	to:         String,
	/// Abbreviated commit SHA the range starts at
	from_short: String,
	/// Abbreviated commit SHA the range ends at
	to_short:   String,
}

impl Range {
	/// Creates a new [`Range`] from [`crate::commit::Commit`].
	pub fn new(from: &Commit, to: &Commit) -> Self {
		Self {
			from:       from.id.clone(),
			to:         to.id.clone(),
			from_short: from.short_id.clone(),
			to_short:   to.short_id.clone(),
		}
	}
}
