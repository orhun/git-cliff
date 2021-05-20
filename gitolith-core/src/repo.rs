use crate::commit::Commit;
use crate::error::{
	Error,
	Result,
};
use git2::{
	Repository as GitRepository,
	Sort,
};
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
			.map(Commit::from)
			.collect())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use git_conventional::ErrorKind;
	use std::env;
	use std::process::Command;
	use std::str;

	fn get_last_commit_hash() -> Result<String> {
		Ok(str::from_utf8(
			Command::new("git")
				.args(&["log", "--pretty=format:'%h'", "-n", "1"])
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
		let last_commit = commits.first().unwrap();
		assert_eq!(Some(get_last_commit_hash()?), last_commit.hash);
		if let Err(e) = last_commit.as_conventional() {
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
