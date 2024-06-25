use crate::config::Remote;
use crate::error::{
	Error,
	Result,
};
use crate::tag::Tag;
use git2::{
	BranchType,
	Commit,
	DescribeOptions,
	Oid,
	Repository as GitRepository,
	Sort,
};
use glob::Pattern;
use indexmap::IndexMap;
use lazy_regex::{
	lazy_regex,
	Lazy,
	Regex,
};
use std::io;
use std::path::PathBuf;
use url::Url;

/// Regex for replacing the signature part of a tag message.
static TAG_SIGNATURE_REGEX: Lazy<Regex> = lazy_regex!(
	r"(?s)-----BEGIN PGP SIGNATURE-----(.*?)-----END PGP SIGNATURE-----"
);

/// Wrapper for [`Repository`] type from git2.
///
/// [`Repository`]: GitRepository
pub struct Repository {
	inner:    GitRepository,
	/// Repository path.
	pub path: PathBuf,
}

impl Repository {
	/// Initializes (opens) the repository.
	pub fn init(path: PathBuf) -> Result<Self> {
		if path.exists() {
			Ok(Self {
				inner: GitRepository::open(&path)?,
				path,
			})
		} else {
			Err(Error::IoError(io::Error::new(
				io::ErrorKind::NotFound,
				"repository path not found",
			)))
		}
	}

	/// Parses and returns the commits.
	///
	/// Sorts the commits by their time.
	pub fn commits(
		&self,
		range: Option<String>,
		include_path: Option<Vec<Pattern>>,
		exclude_path: Option<Vec<Pattern>>,
	) -> Result<Vec<Commit>> {
		let mut revwalk = self.inner.revwalk()?;
		revwalk.set_sorting(Sort::TOPOLOGICAL)?;
		if let Some(range) = range {
			revwalk.push_range(&range)?;
		} else {
			revwalk.push_head()?;
		}
		let mut commits: Vec<Commit> = revwalk
			.filter_map(|id| id.ok())
			.filter_map(|id| self.inner.find_commit(id).ok())
			.collect();
		if include_path.is_some() || exclude_path.is_some() {
			commits.retain(|commit| {
				if let Ok(prev_commit) = commit.parent(0) {
					if let Ok(diff) = self.inner.diff_tree_to_tree(
						commit.tree().ok().as_ref(),
						prev_commit.tree().ok().as_ref(),
						None,
					) {
						return diff
							.deltas()
							.filter_map(|delta| delta.new_file().path())
							.any(|new_file_path| {
								if let Some(include_path) = &include_path {
									include_path
										.iter()
										.any(|glob| glob.matches_path(new_file_path))
								} else if let Some(exclude_path) = &exclude_path {
									!exclude_path
										.iter()
										.any(|glob| glob.matches_path(new_file_path))
								} else {
									false
								}
							});
					}
				}
				false
			});
		}
		Ok(commits)
	}

	/// Returns the current tag.
	///
	/// It is the same as running `git describe --tags`
	pub fn current_tag(&self) -> Option<Tag> {
		self.inner
			.describe(DescribeOptions::new().describe_tags())
			.ok()
			.and_then(|describe| {
				describe
					.format(None)
					.ok()
					.map(|name| self.resolve_tag(&name))
			})
	}

	/// Returns the tag object of the given name.
	///
	/// If given name doesn't exist, it still returns `Tag` with the given name.
	pub fn resolve_tag(&self, name: &str) -> Tag {
		match self
			.inner
			.resolve_reference_from_short_name(name)
			.and_then(|r| r.peel_to_tag())
		{
			Ok(tag) => Tag {
				name:    tag.name().unwrap_or_default().to_owned(),
				message: tag.message().map(|msg| {
					TAG_SIGNATURE_REGEX.replace(msg, "").trim().to_owned()
				}),
			},
			_ => Tag {
				name:    name.to_owned(),
				message: None,
			},
		}
	}

	/// Returns the commit object of the given ID.
	pub fn find_commit(&self, id: String) -> Option<Commit> {
		if let Ok(oid) = Oid::from_str(&id) {
			if let Ok(commit) = self.inner.find_commit(oid) {
				return Some(commit);
			}
		}
		None
	}

	/// Parses and returns a commit-tag map.
	///
	/// It collects lightweight and annotated tags.
	pub fn tags(
		&self,
		pattern: &Option<Regex>,
		topo_order: bool,
	) -> Result<IndexMap<String, Tag>> {
		let mut tags: Vec<(Commit, Tag)> = Vec::new();
		let tag_names = self.inner.tag_names(None)?;
		for name in tag_names
			.iter()
			.flatten()
			.filter(|tag_name| {
				pattern.as_ref().map_or(true, |pat| pat.is_match(tag_name))
			})
			.map(String::from)
		{
			let obj = self.inner.revparse_single(&name)?;
			if let Ok(commit) = obj.clone().into_commit() {
				tags.push((commit, Tag {
					name,
					message: None,
				}));
			} else if let Some(tag) = obj.as_tag() {
				if let Some(commit) = tag
					.target()
					.ok()
					.and_then(|target| target.into_commit().ok())
				{
					tags.push((commit, Tag {
						name:    tag.name().map(String::from).unwrap_or(name),
						message: tag.message().map(|msg| {
							TAG_SIGNATURE_REGEX.replace(msg, "").trim().to_owned()
						}),
					}));
				}
			}
		}
		if !topo_order {
			tags.sort_by(|a, b| a.0.time().seconds().cmp(&b.0.time().seconds()));
		}
		Ok(tags
			.into_iter()
			.map(|(a, b)| (a.id().to_string(), b))
			.collect())
	}

	/// Returns the remote of the upstream repository.
	///
	/// The strategy used here is the following:
	///
	/// Find the branch that HEAD points to, and read the remote configured for
	/// that branch returns the remote and the name of the local branch.
	pub fn upstream_remote(&self) -> Result<Remote> {
		for branch in self.inner.branches(Some(BranchType::Local))? {
			let branch = branch?.0;
			if branch.is_head() {
				let upstream = &self.inner.branch_upstream_remote(&format!(
					"refs/heads/{}",
					&branch.name()?.ok_or_else(|| Error::RepoError(
						String::from("branch name is not valid")
					))?
				))?;
				let upstream_name = upstream.as_str().ok_or_else(|| {
					Error::RepoError(String::from(
						"name of the upstream remote is not valid",
					))
				})?;
				let origin = &self.inner.find_remote(upstream_name)?;
				let url = origin
					.url()
					.ok_or_else(|| {
						Error::RepoError(String::from(
							"failed to get the remote URL",
						))
					})?
					.to_string();
				trace!("Upstream URL: {url}");
				let url = Url::parse(&url)?;
				let segments: Vec<&str> = url
					.path_segments()
					.ok_or_else(|| {
						Error::RepoError(String::from("failed to get URL segments"))
					})?
					.rev()
					.collect();
				if let (Some(owner), Some(repo)) =
					(segments.get(1), segments.first())
				{
					return Ok(Remote {
						owner: owner.to_string(),
						repo:  repo.trim_end_matches(".git").to_string(),
						token: None,
					});
				}
			}
		}
		Err(Error::RepoError(String::from("no remotes configured")))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit as AppCommit;
	use std::env;
	use std::process::Command;
	use std::str;

	fn get_last_commit_hash() -> Result<String> {
		Ok(str::from_utf8(
			Command::new("git")
				.args(["log", "--pretty=format:'%H'", "-n", "1"])
				.output()?
				.stdout
				.as_ref(),
		)?
		.trim_matches('\'')
		.to_string())
	}

	fn get_last_tag() -> Result<String> {
		Ok(str::from_utf8(
			Command::new("git")
				.args(["describe", "--abbrev=0"])
				.output()?
				.stdout
				.as_ref(),
		)?
		.trim()
		.to_string())
	}

	fn get_repository() -> Result<Repository> {
		Repository::init(
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.parent()
				.expect("parent directory not found")
				.to_path_buf(),
		)
	}

	#[test]
	fn get_latest_commit() -> Result<()> {
		let repository = get_repository()?;
		let commits = repository.commits(None, None, None)?;
		let last_commit =
			AppCommit::from(&commits.first().expect("no commits found").clone());
		assert_eq!(get_last_commit_hash()?, last_commit.id);
		Ok(())
	}

	#[test]
	fn get_latest_tag() -> Result<()> {
		let repository = get_repository()?;
		let tags = repository.tags(&None, false)?;
		assert_eq!(get_last_tag()?, tags.last().expect("no tags found").1.name);
		Ok(())
	}

	#[test]
	fn git_tags() -> Result<()> {
		let repository = get_repository()?;
		let tags = repository.tags(&None, true)?;
		assert_eq!(
			tags.get("2b8b4d3535f29231e05c3572e919634b9af907b6")
				.expect(
					"the commit hash does not exist in the repository (tag v0.1.0)"
				)
				.name,
			"v0.1.0"
		);
		assert_eq!(
			tags.get("4ddef08debfff48117586296e49d5caa0800d1b5")
				.expect(
					"the commit hash does not exist in the repository (tag \
					 v0.1.0-beta.4)"
				)
				.name,
			"v0.1.0-beta.4"
		);
		let tags = repository.tags(
			&Some(
				Regex::new("^v[0-9]+\\.[0-9]+\\.[0-9]$")
					.expect("the regex is not valid"),
			),
			true,
		)?;
		assert_eq!(
			tags.get("2b8b4d3535f29231e05c3572e919634b9af907b6")
				.expect(
					"the commit hash does not exist in the repository (tag v0.1.0)"
				)
				.name,
			"v0.1.0"
		);
		assert!(!tags.contains_key("4ddef08debfff48117586296e49d5caa0800d1b5"));
		Ok(())
	}

	#[test]
	fn git_upstream_remote() -> Result<()> {
		let repository = get_repository()?;
		let remote = repository.upstream_remote()?;
		assert_eq!(
			Remote {
				owner: String::from("orhun"),
				repo:  String::from("git-cliff"),
				token: None,
			},
			remote
		);
		Ok(())
	}

	#[test]
	fn resolves_existing_tag_with_name_and_message() -> Result<()> {
		let repository = get_repository()?;
		let tag = repository.resolve_tag("v0.2.3");
		assert_eq!(tag.name, "v0.2.3");
		assert_eq!(
			tag.message,
			Some(
				"Release v0.2.3\n\nBug Fixes\n- Fetch the dependencies before \
				 copying the file to embed (9e29c95)"
					.to_string()
			)
		);

		Ok(())
	}

	#[test]
	fn resolves_tag_when_no_tags_exist() -> Result<()> {
		let repository = get_repository()?;
		let tag = repository.resolve_tag("nonexistent-tag");
		assert_eq!(tag.name, "nonexistent-tag");
		assert_eq!(tag.message, None);
		Ok(())
	}
}
