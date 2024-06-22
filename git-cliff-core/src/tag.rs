/// Common tag object that is parsed from a repository.
///
/// Lightweight tags will have `None` as message.
#[derive(Debug)]
pub struct Tag {
	/// The name of the tag
	pub name:    String,
	/// The message of the tag (only if it was annotated).
	pub message: Option<String>,
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn create_tag_with_name_and_message() {
		let tag = Tag {
			name:    String::from("v1.0"),
			message: Some(String::from("Initial release")),
		};
		assert_eq!(tag.name, "v1.0");
		assert_eq!(tag.message, Some(String::from("Initial release")));
	}

	#[test]
	fn create_tag_with_name_and_no_message() {
		let tag = Tag {
			name:    String::from("v1.0"),
			message: None,
		};
		assert_eq!(tag.name, "v1.0");
		assert_eq!(tag.message, None);
	}

	#[test]
	fn debug_print_tag_with_message() {
		let tag = Tag {
			name:    String::from("v1.0"),
			message: Some(String::from("Initial release")),
		};
		assert_eq!(
			format!("{:?}", tag),
			"Tag { name: \"v1.0\", message: Some(\"Initial release\") }"
		);
	}
}
