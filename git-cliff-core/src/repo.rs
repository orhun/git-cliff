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
	TreeWalkMode,
	Worktree,
};
use glob::Pattern;
use indexmap::IndexMap;
use lazy_regex::{
	Lazy,
	Regex,
	lazy_regex,
};
use std::io;
use std::path::{
	Path,
	PathBuf,
};
use std::result::Result as StdResult;
use url::Url;

/// Regex for replacing the signature part of a tag message.
static TAG_SIGNATURE_REGEX: Lazy<Regex> = lazy_regex!(
	// https://git-scm.com/docs/gitformat-signature#_description
	r"(?s)-----BEGIN (PGP|SSH|SIGNED) (SIGNATURE|MESSAGE)-----(.*?)-----END (PGP|SSH|SIGNED) (SIGNATURE|MESSAGE)-----"
);

/// Name of the cache file for changed files.
const CHANGED_FILES_CACHE: &str = "changed_files_cache";

/// Wrapper for [`Repository`] type from git2.
///
/// [`Repository`]: GitRepository
pub struct Repository {
	inner:                    GitRepository,
	/// Repository path.
	path:                     PathBuf,
	/// Cache path for the changed files of the commits.
	changed_files_cache_path: PathBuf,
}

/// Range of commits in a submodule.
pub struct SubmoduleRange {
	/// Repository object to which this range belongs.
	pub repository: Repository,
	/// Commit range in "<first_submodule_commit>..<last_submodule_commit>" or
	/// "<last_submodule_commit>" format.
	pub range:      String,
}

impl Repository {
	/// Initializes (opens) the repository.
	pub fn init(path: PathBuf) -> Result<Self> {
		if path.exists() {
			let inner = GitRepository::discover(&path).or_else(|err| {
				let jujutsu_path =
					path.join(".jj").join("repo").join("store").join("git");
				if jujutsu_path.exists() {
					GitRepository::open_bare(&jujutsu_path)
				} else {
					Err(err)
				}
			})?;
			let changed_files_cache_path = inner
				.path()
				.join(env!("CARGO_PKG_NAME"))
				.join(CHANGED_FILES_CACHE);
			Ok(Self {
				inner,
				path,
				changed_files_cache_path,
			})
		} else {
			Err(Error::IoError(io::Error::new(
				io::ErrorKind::NotFound,
				"repository path not found",
			)))
		}
	}

	/// Returns the path of the repository.
	pub fn root_path(&self) -> Result<PathBuf> {
		let mut path = if self.inner.is_worktree() {
			let worktree = Worktree::open_from_repository(&self.inner)?;
			worktree.path().to_path_buf()
		} else {
			self.inner.path().to_path_buf()
		};
		if path.ends_with(".git") {
			path.pop();
		}
		Ok(path)
	}

	/// Returns the initial path of the repository.
	///
	/// In case of a submodule this is the relative path to the toplevel
	/// repository.
	pub fn path(&self) -> &PathBuf {
		&self.path
	}

	/// Sets the range for the commit search.
	///
	/// When a single SHA is provided as the range, start from the
	/// root.
	fn set_commit_range(
		revwalk: &mut git2::Revwalk<'_>,
		range: Option<&str>,
	) -> StdResult<(), git2::Error> {
		if let Some(range) = range {
			if range.contains("..") {
				revwalk.push_range(range)?;
			} else {
				revwalk.push(Oid::from_str(range)?)?;
			}
		} else {
			revwalk.push_head()?;
		}
		Ok(())
	}

	/// Parses and returns the commits.
	///
	/// Sorts the commits by their time.
	pub fn commits(
		&self,
		range: Option<&str>,
		include_path: Option<Vec<Pattern>>,
		exclude_path: Option<Vec<Pattern>>,
		topo_order_commits: bool,
	) -> Result<Vec<Commit>> {
		let mut revwalk = self.inner.revwalk()?;
		if topo_order_commits {
			revwalk.set_sorting(Sort::TOPOLOGICAL)?;
		} else {
			revwalk.set_sorting(Sort::TIME)?;
		}

		Self::set_commit_range(&mut revwalk, range).map_err(|e| {
			Error::SetCommitRangeError(
				range.map(String::from).unwrap_or_else(|| "?".to_string()),
				e,
			)
		})?;
		let mut commits: Vec<Commit> = revwalk
			.filter_map(|id| id.ok())
			.filter_map(|id| self.inner.find_commit(id).ok())
			.collect();
		if include_path.is_some() || exclude_path.is_some() {
			let include_patterns = include_path.map(|patterns| {
				patterns.into_iter().map(Self::normalize_pattern).collect()
			});
			let exclude_patterns = exclude_path.map(|patterns| {
				patterns.into_iter().map(Self::normalize_pattern).collect()
			});
			commits.retain(|commit| {
				self.should_retain_commit(
					commit,
					&include_patterns,
					&exclude_patterns,
				)
			});
		}
		Ok(commits)
	}

	/// Returns submodule repositories for a given commit range.
	///
	/// For one or two given commits in this repository, a list of changed
	/// submodules is calculated. If only one commit is given, then all
	/// submodule commits up to the referenced commit will be included. This is
	/// usually the case if a submodule is added to the repository.
	///
	///  For each submodule a [`SubmoduleRange`] object is created
	///
	/// This can then be used to query the submodule's commits by using
	/// [`Repository::commits`].
	pub fn submodules_range(
		&self,
		old_commit: Option<Commit<'_>>,
		new_commit: Commit<'_>,
	) -> Result<Vec<SubmoduleRange>> {
		let old_tree = old_commit.and_then(|commit| commit.tree().ok());
		let new_tree = new_commit.tree().ok();
		let diff = self.inner.diff_tree_to_tree(
			old_tree.as_ref(),
			new_tree.as_ref(),
			None,
		)?;
		// iterate through all diffs and accumulate old/new commit ids
		let before_and_after_deltas = diff.deltas().filter_map(|delta| {
			let old_file_id = delta.old_file().id();
			let new_file_id = delta.new_file().id();
			let range = if old_file_id == new_file_id || new_file_id.is_zero() {
				// no changes or submodule removed
				None
			} else if old_file_id.is_zero() {
				// submodule added
				Some(new_file_id.to_string())
			} else {
				// submodule updated
				Some(format!("{}..{}", old_file_id, new_file_id))
			};
			trace!("Release commit range for submodules: {:?}", range);
			delta.new_file().path().and_then(Path::to_str).zip(range)
		});
		// iterate through all path diffs and find corresponding submodule if
		// possible
		let submodule_range = before_and_after_deltas.filter_map(|(path, range)| {
			let repository = self
				.inner
				.find_submodule(path)
				.ok()
				.and_then(|submodule| Self::init(submodule.path().into()).ok());
			repository.map(|repository| SubmoduleRange { repository, range })
		});
		Ok(submodule_range.collect())
	}

	/// Normalizes the glob pattern to match the git diff paths.
	///
	/// It removes the leading `./` and adds `**` to the end if the pattern is a
	/// directory.
	fn normalize_pattern(pattern: Pattern) -> Pattern {
		let star_added = match pattern.as_str().chars().last() {
			Some('/' | '\\') => Pattern::new(&format!("{pattern}**"))
				.expect("failed to add '**' to the end of glob"),
			_ => pattern,
		};
		match star_added.as_str().strip_prefix("./") {
			Some(stripped) => Pattern::new(stripped)
				.expect("failed to remove leading ./ from glob"),
			None => star_added,
		}
	}

	/// Calculates whether the commit should be retained or not.
	///
	/// This function is used to filter the commits based on the changed files,
	/// and include/exclude patterns.
	fn should_retain_commit(
		&self,
		commit: &Commit,
		include_patterns: &Option<Vec<Pattern>>,
		exclude_patterns: &Option<Vec<Pattern>>,
	) -> bool {
		let changed_files = self.commit_changed_files(commit);
		match (include_patterns, exclude_patterns) {
			(Some(include_pattern), Some(exclude_pattern)) => {
				// check if the commit has any changed files that match any of the
				// include patterns and non of the exclude patterns.
				changed_files.iter().any(|path| {
					include_pattern
						.iter()
						.any(|pattern| pattern.matches_path(path)) &&
						!exclude_pattern
							.iter()
							.any(|pattern| pattern.matches_path(path))
				})
			}
			(Some(include_pattern), None) => {
				// check if the commit has any changed files that match the include
				// patterns.
				changed_files.iter().any(|path| {
					include_pattern
						.iter()
						.any(|pattern| pattern.matches_path(path))
				})
			}
			(None, Some(exclude_pattern)) => {
				// check if the commit has at least one changed file that does not
				// match all exclude patterns.
				changed_files.iter().any(|path| {
					!exclude_pattern
						.iter()
						.any(|pattern| pattern.matches_path(path))
				})
			}
			(None, None) => true,
		}
	}

	/// Returns the changed files of the commit.
	///
	/// It uses a cache to speed up checks to store the changed files of the
	/// commits under `./.git/git-cliff-core/changed_files_cache`. The speed-up
	/// was measured to be around 260x for large repositories.
	///
	/// If the cache is not found, it calculates the changed files and adds them
	/// to the cache via [`Self::commit_changed_files_no_cache`].
	fn commit_changed_files(&self, commit: &Commit) -> Vec<PathBuf> {
		// Cache key is generated from the repository path and commit id
		let cache_key = format!("commit_id:{}", commit.id());

		// Check the cache first.
		{
			if let Ok(result) =
				cacache::read_sync(&self.changed_files_cache_path, &cache_key)
			{
				if let Ok((files, _)) =
					bincode::decode_from_slice(&result, bincode::config::standard())
				{
					return files;
				}
			}
		}

		// If the cache is not found, calculate the result and set it to the cache.
		let result = self.commit_changed_files_no_cache(commit);
		match bincode::encode_to_vec(
			self.commit_changed_files_no_cache(commit),
			bincode::config::standard(),
		) {
			Ok(v) => {
				if let Err(e) = cacache::write_sync_with_algo(
					cacache::Algorithm::Xxh3,
					&self.changed_files_cache_path,
					cache_key,
					v,
				) {
					error!("Failed to set cache for repo {:?}: {e}", self.path);
				}
			}
			Err(e) => {
				error!("Failed to serialize cache for repo {:?}: {e}", self.path);
			}
		}

		result
	}

	/// Calculate the changed files of the commit.
	///
	/// This function does not use the cache (directly calls git2).
	fn commit_changed_files_no_cache(&self, commit: &Commit) -> Vec<PathBuf> {
		let mut changed_files = Vec::new();
		if let Ok(prev_commit) = commit.parent(0) {
			// Compare the current commit with the previous commit to get the
			// changed files.
			// libgit2 does not provide a way to get the changed files directly, so
			// the full diff is calculated here.
			if let Ok(diff) = self.inner.diff_tree_to_tree(
				commit.tree().ok().as_ref(),
				prev_commit.tree().ok().as_ref(),
				None,
			) {
				changed_files.extend(
					diff.deltas().filter_map(|delta| {
						delta.new_file().path().map(PathBuf::from)
					}),
				);
			}
		} else {
			// If there is no parent, it is the first commit.
			// So get all the files in the tree.
			if let Ok(tree) = commit.tree() {
				tree.walk(TreeWalkMode::PreOrder, |dir, entry| {
					if entry.kind().expect("failed to get entry kind") !=
						git2::ObjectType::Blob
					{
						return 0;
					}
					let name = entry.name().expect("failed to get entry name");
					let entry_path = if dir == "," {
						name.to_string()
					} else {
						format!("{dir}/{name}")
					};
					changed_files.push(entry_path.into());
					0
				})
				.expect("failed to get the changed files of the first commit");
			}
		}
		changed_files
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
	pub fn find_commit(&self, id: &str) -> Option<Commit> {
		if let Ok(oid) = Oid::from_str(id) {
			if let Ok(commit) = self.inner.find_commit(oid) {
				return Some(commit);
			}
		}
		None
	}

	/// Decide whether to include tag.
	///
	/// `head_commit` is the `latest` commit to generate changelog. It can be a
	/// branch head or a detached head. `tag_commit` is a tagged commit. If the
	/// commit is in the descendant graph of the `head_commit` or is the
	/// `head_commit` itself, Changelog should include the tag.
	fn should_include_tag(
		&self,
		head_commit: &Commit,
		tag_commit: &Commit,
	) -> Result<bool> {
		Ok(self
			.inner
			.graph_descendant_of(head_commit.id(), tag_commit.id())? ||
			head_commit.id() == tag_commit.id())
	}

	/// Parses and returns a commit-tag map.
	///
	/// It collects lightweight and annotated tags.
	pub fn tags(
		&self,
		pattern: &Option<Regex>,
		topo_order: bool,
		use_branch_tags: bool,
	) -> Result<IndexMap<String, Tag>> {
		let mut tags: Vec<(Commit, Tag)> = Vec::new();
		let tag_names = self.inner.tag_names(None)?;
		let head_commit = self.inner.head()?.peel_to_commit()?;
		for name in tag_names
			.iter()
			.flatten()
			.filter(|tag_name| {
				pattern.as_ref().is_none_or(|pat| pat.is_match(tag_name))
			})
			.map(String::from)
		{
			let obj = self.inner.revparse_single(&name)?;
			if let Ok(commit) = obj.clone().into_commit() {
				if use_branch_tags &&
					!self.should_include_tag(&head_commit, &commit)?
				{
					continue;
				}

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
					if use_branch_tags &&
						!self.should_include_tag(&head_commit, &commit)?
					{
						continue;
					}
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
	///
	/// Note: HEAD must not be detached.
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
				return find_remote(&url);
			}
		}
		Err(Error::RepoError(String::from(
			"no remotes configured or HEAD is detached",
		)))
	}
}

fn find_remote(url: &str) -> Result<Remote> {
	url_path_segments(url).or_else(|err| {
		if url.contains("@") && url.contains(":") && url.contains("/") {
			ssh_path_segments(url)
		} else {
			Err(err)
		}
	})
}

/// Returns the Remote from parsing the HTTPS format URL.
///
/// This function expects the URL to be in the following format:
///
/// > https://hostname/query/path.git
fn url_path_segments(url: &str) -> Result<Remote> {
	let parsed_url = Url::parse(url.strip_suffix(".git").unwrap_or(url))?;
	let segments: Vec<&str> = parsed_url
		.path_segments()
		.ok_or_else(|| Error::RepoError(String::from("failed to get URL segments")))?
		.rev()
		.collect();
	let [repo, owner, ..] = &segments[..] else {
		return Err(Error::RepoError(String::from(
			"failed to get the owner and repo",
		)));
	};
	Ok(Remote {
		owner:      owner.to_string(),
		repo:       repo.to_string(),
		token:      None,
		is_custom:  false,
		api_url:    None,
		native_tls: None,
	})
}

/// Returns the Remote from parsing the SSH format URL.
///
/// This function expects the URL to be in the following format:
///
/// > git@hostname:owner/repo.git
fn ssh_path_segments(url: &str) -> Result<Remote> {
	let [_, owner_repo, ..] = url
		.strip_suffix(".git")
		.unwrap_or(url)
		.split(":")
		.collect::<Vec<_>>()[..]
	else {
		return Err(Error::RepoError(String::from(
			"failed to get the owner and repo from ssh remote (:)",
		)));
	};
	let [owner, repo] = owner_repo.split("/").collect::<Vec<_>>()[..] else {
		return Err(Error::RepoError(String::from(
			"failed to get the owner and repo from ssh remote (/)",
		)));
	};
	Ok(Remote {
		owner:      owner.to_string(),
		repo:       repo.to_string(),
		token:      None,
		is_custom:  false,
		api_url:    None,
		native_tls: None,
	})
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::Commit as AppCommit;
	use std::process::Command;
	use std::str;
	use std::{
		env,
		fs,
	};
	use temp_dir::TempDir;

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

	fn get_root_commit_hash() -> Result<String> {
		Ok(str::from_utf8(
			Command::new("git")
				.args(["rev-list", "--max-parents=0", "HEAD"])
				.output()?
				.stdout
				.as_ref(),
		)?
		.trim_ascii_end()
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
	fn http_url_repo_owner() -> Result<()> {
		let url = "https://hostname.com/bob/magic.git";
		let remote = find_remote(url)?;
		assert_eq!(remote.owner, "bob", "match owner");
		assert_eq!(remote.repo, "magic", "match repo");
		Ok(())
	}

	#[test]
	fn ssh_url_repo_owner() -> Result<()> {
		let url = "git@hostname.com:bob/magic.git";
		let remote = find_remote(url)?;
		assert_eq!(remote.owner, "bob", "match owner");
		assert_eq!(remote.repo, "magic", "match repo");
		Ok(())
	}

	#[test]
	fn get_latest_commit() -> Result<()> {
		let repository = get_repository()?;
		let commits = repository.commits(None, None, None, false)?;
		let last_commit =
			AppCommit::from(&commits.first().expect("no commits found").clone());
		assert_eq!(get_last_commit_hash()?, last_commit.id);
		Ok(())
	}

	#[test]
	fn commit_search() -> Result<()> {
		let repository = get_repository()?;
		assert!(
			repository
				.find_commit("e936ed571533ea6c41a1dd2b1a29d085c8dbada5")
				.is_some()
		);
		Ok(())
	}

	#[test]
	fn get_latest_tag() -> Result<()> {
		let repository = get_repository()?;
		let tags = repository.tags(&None, false, false)?;
		let latest = tags.last().expect("no tags found").1.name.clone();
		assert_eq!(get_last_tag()?, latest);

		let current = repository.current_tag().expect("a current tag").name;
		assert!(current.contains(&latest));
		Ok(())
	}

	#[test]
	fn git_tags() -> Result<()> {
		let repository = get_repository()?;
		let tags = repository.tags(&None, true, false)?;
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
			false,
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
				owner:      remote.owner.clone(),
				repo:       String::from("git-cliff"),
				token:      None,
				is_custom:  false,
				api_url:    remote.api_url.clone(),
				native_tls: None,
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

	#[test]
	fn includes_root_commit() -> Result<()> {
		let repository = get_repository()?;
		// a close descendant of the root commit
		let range = Some("eea3914c7ab07472841aa85c36d11bdb2589a234");
		let commits = repository.commits(range, None, None, false)?;
		let root_commit =
			AppCommit::from(&commits.last().expect("no commits found").clone());
		assert_eq!(get_root_commit_hash()?, root_commit.id);
		Ok(())
	}

	fn create_temp_repo() -> (Repository, TempDir) {
		let temp_dir =
			TempDir::with_prefix("git-cliff-").expect("failed to create temp dir");

		let output = Command::new("git")
			.args(["init"])
			.current_dir(temp_dir.path())
			.output()
			.expect("failed to execute git init");
		assert!(output.status.success(), "git init failed {:?}", output);

		let repo = Repository::init(temp_dir.path().to_path_buf())
			.expect("failed to init repo");
		let output = Command::new("git")
			.args(["config", "user.email", "test@gmail.com"])
			.current_dir(temp_dir.path())
			.output()
			.expect("failed to execute git config user.email");
		assert!(
			output.status.success(),
			"git config user.email failed {:?}",
			output
		);

		let output = Command::new("git")
			.args(["config", "user.name", "test"])
			.current_dir(temp_dir.path())
			.output()
			.expect("failed to execute git config user.name");
		assert!(
			output.status.success(),
			"git config user.name failed {:?}",
			output
		);

		(repo, temp_dir)
	}

	#[test]
	fn open_jujutsu_repo() {
		let (repo, _temp_dir) = create_temp_repo();
		// working copy is the directory that contains the .git directory:
		let working_copy = repo.path;

		// Make the Git repository bare and set HEAD
		std::process::Command::new("git")
			.args(["config", "core.bare", "true"])
			.current_dir(&working_copy)
			.status()
			.expect("failed to make git repo non-bare");
		// Move the Git repo into jj
		let store = working_copy.join(".jj").join("repo").join("store");
		fs::create_dir_all(&store).expect("failed to create dir");
		fs::rename(working_copy.join(".git"), store.join("git"))
			.expect("failed to move git repo");

		// Open repo from working copy, that contains the .jj directory
		let repo = Repository::init(working_copy).expect("failed to init repo");

		// macOS canonical path for temp directories is in /private
		// libgit2 forces the path to be canonical regardless of what we pass in
		if repo.inner.path().starts_with("/private") {
			assert_eq!(
				repo.inner.path().strip_prefix("/private"),
				store.join("git").strip_prefix("/"),
				"open git repo in .jj/repo/store/"
			);
		} else {
			assert_eq!(
				repo.inner.path(),
				store.join("git"),
				"open git repo in .jj/repo/store/"
			);
		}
	}

	#[test]
	fn propagate_error_if_no_repo_found() {
		let temp_dir =
			TempDir::with_prefix("git-cliff-").expect("failed to create temp dir");

		let path = temp_dir.path().to_path_buf();

		let result = Repository::init(path.clone());

		assert!(result.is_err());
		if let Err(error) = result {
			assert!(
				format!("{error:?}").contains(
					format!("could not find repository at '{}'", path.display())
						.as_str()
				)
			)
		}
	}

	fn create_commit_with_files<'a>(
		repo: &'a Repository,
		files: Vec<(&'a str, &'a str)>,
	) -> Commit<'a> {
		for (path, content) in files {
			if let Some(parent) = repo.path.join(path).parent() {
				std::fs::create_dir_all(parent).expect("failed to create dir");
			}
			std::fs::write(repo.path.join(path), content)
				.expect("failed to write file");
		}

		let output = Command::new("git")
			.args(["add", "."])
			.current_dir(&repo.path)
			.output()
			.expect("failed to execute git add");
		assert!(output.status.success(), "git add failed {:?}", output);

		let output = Command::new("git")
			.args(["commit", "--no-gpg-sign", "-m", "test commit"])
			.current_dir(&repo.path)
			.output()
			.expect("failed to execute git commit");
		assert!(output.status.success(), "git commit failed {:?}", output);

		repo.inner
			.head()
			.and_then(|head| head.peel_to_commit())
			.expect("failed to get the last commit")
	}

	#[test]
	fn test_should_retain_commit() {
		let (repo, _temp_dir) = create_temp_repo();

		let new_pattern = |input: &str| {
			Repository::normalize_pattern(
				Pattern::new(input).expect("valid pattern"),
			)
		};

		let first_commit = create_commit_with_files(&repo, vec![
			("initial.txt", "initial content"),
			("dir/initial.txt", "initial content"),
		]);

		{
			let retain = repo.should_retain_commit(
				&first_commit,
				&Some(vec![new_pattern("dir/")]),
				&None,
			);
			assert!(retain, "include: dir/");
		}

		let commit = create_commit_with_files(&repo, vec![
			("file1.txt", "content1"),
			("file2.txt", "content2"),
			("dir/file3.txt", "content3"),
			("dir/subdir/file4.txt", "content4"),
		]);

		{
			let retain = repo.should_retain_commit(&commit, &None, &None);
			assert!(retain, "no include/exclude patterns");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("./")]),
				&None,
			);
			assert!(retain, "include: ./");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("**")]),
				&None,
			);
			assert!(retain, "include: **");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("*")]),
				&None,
			);
			assert!(retain, "include: *");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("dir/")]),
				&None,
			);
			assert!(retain, "include: dir/");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("dir/*")]),
				&None,
			);
			assert!(retain, "include: dir/*");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("file1.txt")]),
				&None,
			);
			assert!(retain, "include: file1.txt");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&None,
				&Some(vec![new_pattern("file1.txt")]),
			);
			assert!(retain, "exclude: file1.txt");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&Some(vec![new_pattern("file1.txt")]),
				&Some(vec![new_pattern("file2.txt")]),
			);
			assert!(retain, "include: file1.txt, exclude: file2.txt");
		}

		{
			let retain = repo.should_retain_commit(
				&commit,
				&None,
				&Some(vec![new_pattern("**/*.txt")]),
			);
			assert!(!retain, "exclude: **/*.txt");
		}
	}
}
