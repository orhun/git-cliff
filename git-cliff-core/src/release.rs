use crate::commit::Commit;

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
	/// Commit ID of the tag.
	#[serde(rename = "commit_id")]
	pub commit_id: Option<String>,
	/// Timestamp of the release.
	pub timestamp: i64,
	/// Previous release.
	pub previous:  Option<Box<Release<'a>>>,
}
