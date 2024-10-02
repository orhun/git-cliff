use git_cliff_core::commit::{Commit, Signature};
use git_cliff_core::config::{
	ChangelogConfig, CommitParser, GitConfig, LinkParser, TextProcessor,
};
use git_cliff_core::error::Result;
use git_cliff_core::release::*;
use git_cliff_core::template::Template;
use pretty_assertions::assert_eq;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;

#[test]
fn generate_changelog() -> Result<()> {
	let changelog_config = ChangelogConfig {
		header: Some(String::from("this is a changelog")),
		body: Some(String::from(
			r#"
## Release {{ version }} - <DATE>
{% for group, commits in commits | group_by(attribute="group") %}
### {{ group }}
{% for commit in commits %}
{%- if commit.scope -%}
- *({{commit.scope}})* {{ commit.message }}{%- if commit.links %} ({% for link in commit.links %}[{{link.text}}]({{link.href}}) {% endfor -%}){% endif %}
{% else -%}
- {{ commit.message }}
{% endif -%}
{% if commit.breaking -%}
{% raw %}  {% endraw %}- **BREAKING**: {{commit.breaking_description}}
{% endif -%}
{% endfor -%}
{% endfor %}"#,
		)),
		footer: Some(String::from("eoc - end of changelog")),
		trim: None,
		render_always: None,
		postprocessors: None,
		output: None,
	};
	let git_config = GitConfig {
		conventional_commits: Some(true),
		filter_unconventional: Some(true),
		split_commits: Some(false),
		commit_preprocessors: Some(vec![TextProcessor {
			pattern: Regex::new(r"\(fixes (#[1-9]+)\)").unwrap(),
			replace: Some(String::from("[closes Issue${1}]")),
			replace_command: None,
		}]),
		commit_parsers: Some(vec![
			CommitParser {
				sha: Some(String::from("coffee")),
				message: None,
				body: None,
				footer: None,
				group: Some(String::from("I love coffee")),
				default_scope: None,
				scope: None,
				skip: None,
				field: None,
				pattern: None,
			},
			CommitParser {
				sha: None,
				message: Regex::new("^feat").ok(),
				body: None,
				footer: None,
				group: Some(String::from("shiny features")),
				default_scope: None,
				scope: None,
				skip: None,
				field: None,
				pattern: None,
			},
			CommitParser {
				sha: None,
				message: Regex::new("^fix").ok(),
				body: None,
				footer: None,
				group: Some(String::from("fix bugs")),
				default_scope: None,
				scope: None,
				skip: None,
				field: None,
				pattern: None,
			},
			CommitParser {
				sha: None,
				message: Regex::new("^test").ok(),
				body: None,
				footer: None,
				group: None,
				default_scope: None,
				scope: Some(String::from("tests")),
				skip: None,
				field: None,
				pattern: None,
			},
			CommitParser {
				sha: None,
				message: None,
				body: None,
				footer: None,
				group: Some(String::from("docs")),
				default_scope: None,
				scope: None,
				skip: None,
				field: Some(String::from("author.name")),
				pattern: Regex::new("John Doe").ok(),
			},
		]),
		protect_breaking_commits: None,
		filter_commits: Some(true),
		tag_pattern: None,
		skip_tags: None,
		ignore_tags: None,
		count_tags: None,
		use_branch_tags: None,
		topo_order: None,
		sort_commits: None,
		link_parsers: Some(vec![
			LinkParser {
				pattern: Regex::new("#(\\d+)").unwrap(),
				href: String::from("https://github.com/$1"),
				text: None,
			},
			LinkParser {
				pattern: Regex::new("https://github.com/(.*)").unwrap(),
				href: String::from("https://github.com/$1"),
				text: Some(String::from("$1")),
			},
		]),
		limit_commits: None,
	};

	let mut commit_with_author = Commit::new(
		String::from("hjdfas32"),
		String::from("docs(cool): testing author filtering"),
	);

	commit_with_author.author = Signature {
		name: Some("John Doe".to_string()),
		email: None,
		timestamp: 0x0,
	};

	let releases = vec![
		Release {
			version:   Some(String::from("v2.0.0")),
			message: None,
            extra: None,
			commits:   vec![

				Commit::new(
					String::from("000abc"),
					String::from("Add unconventional commit"),
				),
				Commit::new(String::from("abc123"), String::from("feat: add xyz")),
				Commit::new(String::from("abc124"), String::from("feat: add zyx")),
				Commit::new(
					String::from("abc124"),
					String::from(
						"feat(random-scope): add random feature\n\nThis is related to https://github.com/NixOS/nixpkgs/issues/136814\n\nCloses #123",
					),
				),
				Commit::new(String::from("def789"), String::from("invalid commit")),
				Commit::new(
					String::from("def789"),
					String::from("feat(big-feature)!: this is a breaking change"),
				),
				Commit::new(String::from("qwerty"), String::from("fix: fix abc")),
				Commit::new(
					String::from("qwop"),
					String::from("final: invalid commit"),
				),
				Commit::new(
					String::from("hjkl12"),
					String::from("chore: do boring stuff"),
				),
				Commit::new(
					String::from("hjkl13"),
					String::from("test(x): test some stuff"),
				),
				Commit::new(
					String::from("1234"),
					String::from("fix: support preprocessing (fixes #99)"),
				),
                commit_with_author
			]
			.iter()
			.filter_map(|c| c.process(&git_config).ok())
			.collect::<Vec<Commit>>(),
			commit_id: None,
			timestamp: 0,
			previous:  None,
			repository: Some(String::from("/root/repo")),
			#[cfg(feature = "github")]
			github: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "gitlab")]
			gitlab: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "gitea")]
			gitea: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "bitbucket")]
			bitbucket: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
		},
		Release {
			version:   Some(String::from("v1.0.0")),
			message: None,
            extra: None,
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
			previous:  None,
			repository: Some(String::from("/root/repo")),
			#[cfg(feature = "github")]
			github: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "gitlab")]
			gitlab: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "gitea")]
			gitea: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
			#[cfg(feature = "bitbucket")]
			bitbucket: git_cliff_core::remote::RemoteReleaseMetadata {
				contributors: vec![],
			},
		},
	];

	let out = &mut String::new();
	let template = Template::new("test", changelog_config.body.unwrap(), false)?;

	writeln!(out, "{}", changelog_config.header.unwrap()).unwrap();
	for release in releases {
		write!(
			out,
			"{}",
			template.render(
				&release,
				Option::<HashMap<&str, String>>::None.as_ref(),
				&[TextProcessor {
					pattern: Regex::new("<DATE>").unwrap(),
					replace: Some(String::from("2023")),
					replace_command: None,
				}]
			)?
		)
		.unwrap();
	}
	writeln!(out, "{}", changelog_config.footer.unwrap()).unwrap();

	assert_eq!(
		r#"this is a changelog

## Release v2.0.0 - 2023

### docs
- *(cool)* testing author filtering

### fix bugs
- fix abc
- support preprocessing [closes Issue#99]

### shiny features
- add xyz
- add zyx
- *(random-scope)* add random feature ([#123](https://github.com/123) [NixOS/nixpkgs/issues/136814](https://github.com/NixOS/nixpkgs/issues/136814) )
- *(big-feature)* this is a breaking change
  - **BREAKING**: this is a breaking change

### test
- *(tests)* test some stuff

## Release v1.0.0 - 2023

### chore
- do nothing

### feat
- add cool features

### fix
- fix stuff
- fix more stuff
eoc - end of changelog
"#,
		out
	);

	Ok(())
}
