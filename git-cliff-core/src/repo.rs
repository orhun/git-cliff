use crate::error::{
	Error,
	Result,
};
use git2::{
	Commit,
	ObjectType,
	Repository as GitRepository,
	Sort,
};
use std::collections::HashMap;
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
				"path not found",
			)))
		}
	}

	/// Parses and returns the commits.
	///
	/// Sorts the commits by their time.
	pub fn commits(&self) -> Result<Vec<Commit>> {
		let mut revwalk = self.inner.revwalk()?;
		revwalk.set_sorting(Sort::TIME)?;
		revwalk.push_head()?;
		Ok(revwalk
			.filter_map(|id| id.ok())
			.filter_map(|id| self.inner.find_commit(id).ok())
			.collect())
	}

	/// Parses and returns a commit-tag map.
	///
	/// It collects lightweight and annotated tags.
	pub fn tags(&self, pattern: &str) -> Result<HashMap<String, String>> {
		let mut tags = HashMap::new();
		let tag_names = self.inner.tag_names(Some(pattern))?;
		for name in tag_names.iter().flatten().map(String::from) {
			let obj = self.inner.revparse_single(&name)?;
			if let Ok(commit) = obj.clone().into_commit() {
				tags.insert(commit.id().to_string(), name);
			} else if let Some(tag) = obj.as_tag() {
				if let Some(ObjectType::Commit) = tag.target_type() {
					tags.insert(tag.target_id().to_string(), name);
				}
			}
		}
		Ok(tags)
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
				.args(&["log", "--pretty=format:'%H'", "-n", "1"])
				.output()?
				.stdout
				.as_ref(),
		)
		.unwrap()
		.trim_matches('\'')
		.to_string())
	}

	#[test]
	fn git_log() -> Result<()> {
		let repository = Repository::init(
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.parent()
				.unwrap()
				.to_path_buf(),
		)?;
		let commits = repository.commits()?;
		let last_commit = AppCommit::from(&commits.first().unwrap().clone());
		assert_eq!(get_last_commit_hash()?, last_commit.id);
		if let Err(e) = last_commit.into_conventional() {
			match e {
				Error::ParseError(e) => {
					assert_eq!(ErrorKind::InvalidFormat, e.kind())
				}
				_ => {
					unreachable!()
				}
			}
		}
		Ok(())
	}
}
