use crate::error::{
	Error,
	Result,
};
use git2::{
	Commit,
	DescribeOptions,
	Repository as GitRepository,
	Sort,
};
use glob::Pattern;
use indexmap::IndexMap;
use std::io;
use std::path::PathBuf;

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
		range: Option<String>,
		include_path: Option<Vec<Pattern>>,
		exclude_path: Option<Vec<Pattern>>,
	) -> Result<Vec<Commit>> {
		let mut revwalk = self.inner.revwalk()?;
		revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL)?;
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
	pub fn current_tag(&self) -> Option<String> {
		self.inner
			.describe(DescribeOptions::new().describe_tags())
			.ok()
			.and_then(|describe| describe.format(None).ok())
	}

	/// Parses and returns a commit-tag map.
	///
	/// It collects lightweight and annotated tags.
	pub fn tags(
		&self,
		pattern: &Option<String>,
		topo_order: bool,
	) -> Result<IndexMap<String, String>> {
		let mut tags: Vec<(Commit, String)> = Vec::new();
		let tag_names = self.inner.tag_names(pattern.as_deref())?;
		for name in tag_names.iter().flatten().map(String::from) {
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
			tags.sort_by(|a, b| a.0.time().seconds().cmp(&b.0.time().seconds()));
		}
		Ok(tags
			.into_iter()
			.map(|(a, b)| (a.id().to_string(), b))
			.collect())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit as AppCommit;
	use git_conventional::ErrorKind;
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

	#[test]
	fn git_log() -> Result<()> {
		let repository = Repository::init(
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.parent()
				.expect("parent directory not found")
				.to_path_buf(),
		)?;
		let commits = repository.commits(None, None, None)?;
		let last_commit =
			AppCommit::from(&commits.first().expect("no commits found").clone());
		assert_eq!(get_last_commit_hash()?, last_commit.id);
		if let Err(e) = last_commit.into_conventional() {
			match e {
				Error::ParseError(e) => {
					eprintln!("\nthe last commit is not conventional\n");
					assert_eq!(ErrorKind::InvalidFormat, e.kind())
				}
				_ => {
					unreachable!()
				}
			}
		}
		let tags = repository.tags(&None, false)?;
		assert_eq!(&get_last_tag()?, tags.last().expect("no tags found").1);
		Ok(())
	}
}
