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
	pub version:   Option<String>,
	pub commits:   Vec<Commit<'a>>,
	#[serde(rename = "commit_id")]
	pub commit_id: Option<String>,
	pub timestamp: i64,
}
