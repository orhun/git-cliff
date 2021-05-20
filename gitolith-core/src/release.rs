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
pub struct ReleaseRoot {
	pub releases: Vec<Release>,
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
pub struct Release {
	pub version:   Option<String>,
	pub commits:   Vec<String>,
	#[serde(rename = "commit_id")]
	pub commit_id: Option<String>,
}
