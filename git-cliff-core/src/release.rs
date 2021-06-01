use crate::commit::Commit;

/// Root object for deserializing from JSON
#[derive(
	Default,
	Debug,
	Clone,
	PartialEq,
	serde_derive::Serialize,
	serde_derive::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseRoot<'a> {
	pub releases: Vec<Release<'a>>,
}

/// Representation of a release.
#[derive(
	Default,
	Debug,
	Clone,
	PartialEq,
	serde_derive::Serialize,
	serde_derive::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Release<'a> {
	/// Release version, git tag.
	pub version:   Option<String>,
	/// Commits made for the release.
	pub commits:   Vec<Commit<'a>>,
	#[serde(rename = "commit_id")]
	/// Commit ID of the tag.
	pub commit_id: Option<String>,
	/// Timestamp of the release.
	pub timestamp: i64,
}
