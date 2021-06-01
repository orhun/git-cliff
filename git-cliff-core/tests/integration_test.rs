use git_cliff_core::commit::Commit;
use git_cliff_core::config::{
	ChangelogConfig,
	CommitParser,
	GitConfig,
};
use git_cliff_core::error::Result;
use git_cliff_core::release::*;
use git_cliff_core::template::Template;
use pretty_assertions::assert_eq;
use regex::Regex;
use std::fmt::Write;

#[test]
fn generate_changelog() -> Result<()> {
	let changelog_config = ChangelogConfig {
		header: String::from("this is a changelog"),
		body:   String::from(
			r#"
        ## Release {{ version }}
        {% for group, commits in commits | group_by(attribute="group") %}
        ### {{ group }}
        {% for commit in commits %}
        - {{ commit.message }}{% endfor %}
        {% endfor %}"#,
		),
		footer: String::from("eoc - end of changelog"),
	};
	let git_config = GitConfig {
		commit_parsers: vec![
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
		filter_commits: true,
		tag_pattern:    String::new(),
		skip_tags:      Regex::new("v3*").unwrap(),
	};

	let releases = vec![
		Release {
			version:   Some(String::from("v2.0.0")),
			commits:   vec![
				Commit::new(String::from("abc123"), String::from("feat: add xyz")),
				Commit::new(String::from("abc124"), String::from("feat: add zyx")),
				Commit::new(String::from("def789"), String::from("invalid commit")),
				Commit::new(String::from("qwerty"), String::from("fix: fix abc")),
				Commit::new(
					String::from("hjkl12"),
					String::from("chore: do boring stuff"),
				),
			]
			.iter()
			.filter_map(|c| {
				c.process(&git_config.commit_parsers, git_config.filter_commits)
					.ok()
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
				Commit::new(String::from("0werty"), String::from("fix: fix stuff")),
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
	];

	let out = &mut String::new();
	let template = Template::new(changelog_config.body)?;
	if !changelog_config.header.is_empty() {
		writeln!(out, "{}", changelog_config.header).unwrap();
	}
	for release in releases {
		write!(out, "{}", template.render(&release)?).unwrap();
	}
	if !changelog_config.footer.is_empty() {
		writeln!(out, "{}", changelog_config.footer).unwrap();
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
