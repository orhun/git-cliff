use super::models_v2::{
	Config,
	Remote,
};
use crate::config::parsing;
use crate::error::Result;
use std::env;
use std::path::PathBuf;

#[test]
fn parse_config() -> Result<()> {
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.parent()
		.expect("parent directory not found")
		.to_path_buf()
		.join("config")
		.join(crate::DEFAULT_CONFIG);

	const FOOTER_VALUE: &str = "test";
	const RELEASE_TAGS_PATTERN_VALUE: &str = ".*[0-9].*";
	const RELEASE_SKIP_TAGS_PATTERN_VALUE: &str = "v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+";

	env::set_var("GIT_CLIFF__CHANGELOG__FOOTER_TEMPLATE", FOOTER_VALUE);
	env::set_var(
		"GIT_CLIFF__RELEASE__TAGS_PATTERN",
		RELEASE_TAGS_PATTERN_VALUE,
	);
	env::set_var(
		"GIT_CLIFF__RELEASE__SKIP_TAGS_PATTERN",
		RELEASE_SKIP_TAGS_PATTERN_VALUE,
	);

	let config = parsing::parse::<Config>(&path)?;

	assert_eq!(
		Some(String::from(FOOTER_VALUE)),
		config.changelog.footer_template
	);
	assert_eq!(
		Some(String::from(RELEASE_TAGS_PATTERN_VALUE)),
		config
			.release
			.tags_pattern
			.map(|tags_pattern| tags_pattern.to_string())
	);
	assert_eq!(
		Some(String::from(RELEASE_SKIP_TAGS_PATTERN_VALUE)),
		config
			.release
			.skip_tags_pattern
			.map(|skip_tags_pattern| skip_tags_pattern.to_string())
	);
	Ok(())
}

#[test]
fn remote_config() {
	let remote1 = Remote::new("abc", "xyz1");
	let remote2 = Remote::new("abc", "xyz2");
	assert!(!remote1.eq(&remote2));
	assert_eq!("abc/xyz1", remote1.to_string());
	assert!(remote1.is_set());
	assert!(!Remote::new("", "test").is_set());
	assert!(!Remote::new("test", "").is_set());
	assert!(!Remote::new("", "").is_set());
}
