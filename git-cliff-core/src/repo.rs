use crate::config::Remote;
use crate::error::{
	Error,
	Result,
};
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
use regex::Regex;
use std::cmp::Reverse;
use std::io;
use std::path::PathBuf;
use url::Url;

/// Wrapper for [`Repository`] type from git2.
///
/// [`Repository`]: GitRepository
pub struct Repository {
	inner: GitRepository,
}

impl Repository {
	/// Initializes (opens) the repository.
	pub fn init(path: PathBuf) -> Result<Self> {
		if path.exists() {
			Ok(Self {
				inner: GitRepository::open(path)?,
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
		range: Option<&str>,
		include_path: Option<&[Pattern]>,
		exclude_path: Option<&[Pattern]>,
	) -> Result<Vec<Commit>> {
		let mut revwalk = self.inner.revwalk()?;
		revwalk.set_sorting(Sort::TOPOLOGICAL)?;
		if let Some(range) = range {
			revwalk.push_range(range)?;
		} else {
			revwalk.push_head()?;
		}
		let mut commits: Vec<Commit> = revwalk
			.filter_map(|id| id.ok())
			.filter_map(|id| self.inner.find_commit(id).ok())
			.collect();
		if include_path.is_some() || exclude_path.is_some() {
			commits.retain(|commit| {
				let prev_tree = commit
					.parent(0)
					.and_then(|prev_commit| prev_commit.tree())
					.ok();
				let Ok(diff) = self.inner.diff_tree_to_tree(
					commit.tree().ok().as_ref(),
					prev_tree.as_ref(),
					None,
				) else {
					return false;
				};
				diff.deltas()
					.filter_map(|delta| delta.new_file().path())
					.any(|new_file_path| {
						if let Some(include_path) = include_path {
							return include_path
								.iter()
								.any(|glob| glob.matches_path(new_file_path));
						}
						if let Some(exclude_path) = exclude_path {
							return !exclude_path
								.iter()
								.any(|glob| glob.matches_path(new_file_path));
						}
						unreachable!()
					})
			});
		}
		Ok(commits)
	}

	/// Returns the current tag.
	///
	/// It is the same as running `git describe --tags`
	pub fn current_tag(&self) -> Option<String> {
		self.inner
			.describe(DescribeOptions::new().describe_tags())
			.ok()
			.and_then(|describe| describe.format(None).ok())
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
		pattern: Option<&Regex>,
		topo_order: bool,
	) -> Result<TaggedCommits<'_>> {
		let mut tags: Vec<(Commit, String)> = Vec::new();
		let tag_names = self.inner.tag_names(None)?;
		for name in tag_names
			.iter()
			.flatten()
			.filter(|tag_name| {
				pattern.is_none() ||
					pattern.is_some_and(|pat| pat.is_match(tag_name))
			})
			.map(String::from)
		{
			let obj = self.inner.revparse_single(&name)?;
			if let Ok(commit) = obj.clone().into_commit() {
				tags.push((commit, name));
			} else if let Some(tag) = obj.as_tag() {
				if let Some(commit) = tag
					.target()
					.ok()
					.and_then(|target| target.into_commit().ok())
				{
					tags.push((commit, name));
				}
			}
		}
		if !topo_order {
			tags.sort_by_key(|(commit, _)| commit.time().seconds());
		}
		TaggedCommits::new(self, tags)
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

/// Stores which commits are tagged with which tags.
#[derive(Debug)]
pub struct TaggedCommits<'a> {
	/// All the commits in the repository.
	pub commits: IndexMap<String, Commit<'a>>,
	/// Commit ID to tag map.
	tags:        IndexMap<String, String>,
	/// List of tags with their commit index.
	///
	/// Sorted in reverse order, meaning the first element is the latest tag.
	///
	/// Used for lookups.
	tag_ids:     Vec<(usize, String)>,
}

impl<'a> TaggedCommits<'a> {
	fn new(
		repository: &'a Repository,
		tags: Vec<(Commit<'a>, String)>,
	) -> Result<Self> {
		let commits = repository.commits(None, None, None)?;
		let commits: IndexMap<_, _> = commits
			.into_iter()
			.map(|c| (c.id().to_string(), c))
			.collect();
		let mut tag_ids: Vec<_> = tags
			.iter()
			.filter_map(|(commit, tag)| {
				let id = commit.id().to_string();
				let idx = commits.get_index_of(&id)?;
				Some((idx, tag.to_string()))
			})
			.collect();
		tag_ids.sort_by_key(|(idx, _)| Reverse(*idx));
		let tags = tags
			.into_iter()
			.map(|(commit, tag)| (commit.id().to_string(), tag))
			.collect();
		Ok(Self {
			commits,
			tag_ids,
			tags,
		})
	}

	/// Returns the number of tags.
	pub fn len(&self) -> usize {
		self.tags.len()
	}

	/// Returns `true` if there are no tags.
	pub fn is_empty(&self) -> bool {
		self.tags.is_empty()
	}

	/// Returns an iterator over all the tags.
	pub fn tags(&self) -> impl Iterator<Item = &str> {
		self.tags.iter().map(|(_, v)| v.as_str())
	}

	/// Returns the last tag.
	pub fn last(&self) -> Option<&str> {
		self.tags().last()
	}

	/// Returns the tag of the given commit.
	///
	/// Note that this only searches for an exact match.
	/// For a more general search, use [`get_closest`](Self::get_closest)
	/// instead.
	pub fn get(&self, commit: &str) -> Option<&str> {
		self.tags.get(commit).map(String::as_str)
	}

	/// Returns the tag at the given index.
	///
	/// The index can be calculated with `tags().position()`.
	pub fn get_index(&self, idx: usize) -> Option<&str> {
		self.tags.get_index(idx).map(|(_, v)| v.as_str())
	}

	/// Returns the tag closest to the given commit.
	pub fn get_closest(&self, commit: &str) -> Option<&str> {
		if let Some(tagged) = self.get(commit) {
			return Some(tagged);
		}

		let index = self.commits.get_index_of(commit)?;
		let (_, tag) = self.tag_ids.iter().find(|(tag_idx, _)| index >= *tag_idx)?;
		Some(tag)
	}

	/// Returns the commit of the given tag.
	pub fn get_commit(&self, tag: &str) -> Option<&str> {
		self.tags
			.iter()
			.find(|(_, t)| *t == tag)
			.map(|(commit, _)| commit.as_str())
	}

	/// Returns `true` if the given tag exists.
	pub fn contains_commit(&self, commit: &str) -> bool {
		self.tags.contains_key(commit)
	}

	/// Inserts a new tagged commit.
	pub fn insert(&mut self, commit: String, tag: String) {
		if let Some(index) = self.commits.get_index_of(&commit) {
			if let Err(idx) = self.binary_search(index) {
				self.tag_ids.insert(idx, (index, tag.clone()));
			}
		}
		self.tags.insert(commit, tag);
	}

	/// Retains only the tags specified by the predicate.
	pub fn retain(&mut self, mut f: impl FnMut(&str) -> bool) {
		self.tags.retain(|_, tag| f(tag));
		self.tag_ids.retain(|(_, tag)| f(tag));
	}

	fn binary_search(&self, index: usize) -> std::result::Result<usize, usize> {
		self.tag_ids
			.binary_search_by_key(&Reverse(index), |(tag_idx, _)| Reverse(*tag_idx))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit as AppCommit;
	use std::env;
	use std::path::Path;
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
			Path::new(env!("CARGO_MANIFEST_DIR"))
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
		let tags = repository.tags(None, false)?;
		assert_eq!(&get_last_tag()?, tags.tags().last().expect("no tags found"));
		Ok(())
	}

	#[test]
	fn git_tags() -> Result<()> {
		let repository = get_repository()?;
		let tags = repository.tags(None, true)?;
		assert_eq!(
			tags.get("2b8b4d3535f29231e05c3572e919634b9af907b6").expect(
				"the commit hash does not exist in the repository (tag v0.1.0)"
			),
			"v0.1.0"
		);
		assert_eq!(
			tags.get("4ddef08debfff48117586296e49d5caa0800d1b5").expect(
				"the commit hash does not exist in the repository (tag \
				 v0.1.0-beta.4)"
			),
			"v0.1.0-beta.4"
		);
		let tags = repository.tags(
			Some(
				&Regex::new("^v[0-9]+\\.[0-9]+\\.[0-9]$")
					.expect("the regex is not valid"),
			),
			true,
		)?;
		assert_eq!(
			tags.get("2b8b4d3535f29231e05c3572e919634b9af907b6").expect(
				"the commit hash does not exist in the repository (tag v0.1.0)"
			),
			"v0.1.0"
		);
		assert!(!tags.contains_commit("4ddef08debfff48117586296e49d5caa0800d1b5"));
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
}
