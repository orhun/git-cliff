use gitolith_core::changelog::Changelog;
use gitolith_core::commit::Commit;
use gitolith_core::config::{
	ChangelogConfig,
	CommitParser,
};
use gitolith_core::error::Result;
use gitolith_core::release::*;
use pretty_assertions::assert_eq;
use regex::Regex;
use std::fmt::Write;

#[test]
fn generate_changelog() -> Result<()> {
	let config = ChangelogConfig {
		header:          String::from("this is a changelog"),
		body:            String::from(
			r#"
        ## Release {{ version }}
        {% for group, commits in commits | group_by(attribute="group") %}
        ### {{ group }}
        {% for commit in commits %}
        - {{ commit.message }}{% endfor %}
        {% endfor %}"#,
		),
		footer:          String::from("eoc - end of changelog"),
		commit_parsers:  vec![
			CommitParser {
				regex: Regex::new("feat*").unwrap(),
				group: Some(String::from("shiny features")),
				skip:  None,
			},
			CommitParser {
				regex: Regex::new("fix*").unwrap(),
				group: Some(String::from("fix bugs")),
				skip:  None,
			},
		],
		filter_group:    true,
		git_tag_pattern: String::new(),
		skip_tags_regex: Regex::new("v3*").unwrap(),
	};

	let release_root = ReleaseRoot {
		releases: vec![
			Release {
				version:   Some(String::from("v2.0.0")),
				commits:   vec![
					Commit::new(
						String::from("abc123"),
						String::from("feat: add xyz"),
					),
					Commit::new(
						String::from("abc124"),
						String::from("feat: add zyx"),
					),
					Commit::new(
						String::from("def789"),
						String::from("invalid commit"),
					),
					Commit::new(
						String::from("qwerty"),
						String::from("fix: fix abc"),
					),
					Commit::new(
						String::from("hjkl12"),
						String::from("chore: do boring stuff"),
					),
				]
				.iter()
				.filter_map(|c| {
					c.process(&config.commit_parsers, config.filter_group).ok()
				})
				.collect::<Vec<Commit>>(),
				commit_id: None,
				timestamp: 0,
			},
			Release {
				version:   Some(String::from("v1.0.0")),
				commits:   vec![
					Commit::new(
						String::from("0bc123"),
						String::from("feat: add cool features"),
					),
					Commit::new(
						String::from("0werty"),
						String::from("fix: fix stuff"),
					),
					Commit::new(
						String::from("0w3rty"),
						String::from("fix: fix more stuff"),
					),
					Commit::new(
						String::from("0jkl12"),
						String::from("chore: do nothing"),
					),
				]
				.into_iter()
				.filter_map(|c| c.into_conventional().ok())
				.collect::<Vec<Commit>>(),
				commit_id: None,
				timestamp: 0,
			},
		],
	};

	let out = &mut String::new();
	let changelog = Changelog::new(config.body)?;
	if !config.header.is_empty() {
		writeln!(out, "{}", config.header).unwrap();
	}
	for release in release_root.releases {
		write!(out, "{}", changelog.generate(release)?).unwrap();
	}
	if !config.footer.is_empty() {
		writeln!(out, "{}", config.footer).unwrap();
	}

	assert_eq!(
		"this is a changelog

        ## Release v2.0.0
        
        ### fix bugs
        
        - fix abc
        
        ### shiny features
        
        - add xyz
        - add zyx
        
        ## Release v1.0.0
        
        ### chore
        
        - do nothing
        
        ### feat
        
        - add cool features
        
        ### fix
        
        - fix stuff
        - fix more stuff
        eoc - end of changelog\n",
		out
	);

	Ok(())
}
