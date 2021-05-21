use crate::commit::Commit;

/// Context of a template rendering.
#[derive(Debug, Default, serde_derive::Serialize)]
pub struct Context<'a> {
	pub release_title: String,
	pub changes:       Vec<Commit<'a>>,
}
