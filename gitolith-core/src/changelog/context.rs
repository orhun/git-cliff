/// Context of a template rendering.
#[derive(Debug, Default, serde_derive::Serialize)]
pub struct Context {
	pub release_title: String,
	pub changes:       Vec<Change>,
}

/// A single changelog entry.
#[derive(Debug, serde_derive::Serialize)]
pub struct Change {
	pub title:   String,
	pub entries: Vec<String>,
}
