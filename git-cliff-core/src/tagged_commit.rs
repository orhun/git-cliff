//! Tagged commit lookup.

use std::cmp::Reverse;

use git2::Commit;
use indexmap::IndexMap;

use crate::error::Result;
use crate::repo::Repository;
use crate::tag::Tag;

/// Stores which commits are tagged with which tags.
#[derive(Debug)]
pub struct TaggedCommits<'a> {
    commits: IndexMap<String, Commit<'a>>,
    tags: IndexMap<String, Tag>,
    tag_indexes: Vec<usize>,
}

impl<'a> TaggedCommits<'a> {
    pub(crate) fn new(repository: &'a Repository, tags: Vec<(Commit<'a>, Tag)>) -> Result<Self> {
        let commits = repository.commits(None, None, None, false)?;
        let commits: IndexMap<_, _> = commits
            .into_iter()
            .map(|c| (c.id().to_string(), c))
            .collect();
        let mut tag_indexes: Vec<_> = tags
            .iter()
            .filter_map(|(commit, _tag)| {
                let id = commit.id().to_string();
                commits.get_index_of(&id)
            })
            .collect();
        tag_indexes.sort_by_key(|idx| Reverse(*idx));
        let tags = tags
            .into_iter()
            .map(|(commit, tag)| (commit.id().to_string(), tag))
            .collect();
        Ok(Self {
            commits,
            tags,
            tag_indexes,
        })
    }

    /// Returns the number of tags.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Returns `true` if there are no tags.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Returns an iterator over all the tags.
    pub fn tags(&self) -> impl Iterator<Item = &Tag> {
        self.tags.iter().map(|(_, tag)| tag)
    }

    /// Returns the last tag.
    #[must_use]
    pub fn last(&self) -> Option<&Tag> {
        self.tags().last()
    }

    /// Returns the tag of the given commit.
    ///
    /// Note that this only searches for an exact match. For a more general
    /// search, use [`get_closest`](Self::get_closest) instead.
    #[must_use]
    pub fn get(&self, commit: &str) -> Option<&Tag> {
        self.tags.get(commit)
    }

    /// Returns the tag at the given index.
    ///
    /// The index can be calculated with `tags().position()`.
    #[must_use]
    pub fn get_index(&self, idx: usize) -> Option<&Tag> {
        self.tags.get_index(idx).map(|(_, tag)| tag)
    }

    /// Returns the tag closest to the given commit.
    #[must_use]
    pub fn get_closest(&self, commit: &str) -> Option<&Tag> {
        if let Some(tagged) = self.get(commit) {
            return Some(tagged);
        }

        let index = self.commits.get_index_of(commit)?;
        let tag_index = *self.tag_indexes.iter().find(|tag_idx| index >= **tag_idx)?;
        self.get_tag_by_index(tag_index)
    }

    fn get_tag_by_index(&self, index: usize) -> Option<&Tag> {
        let (commit_of_tag, _) = self.commits.get_index(index)?;
        self.tags.get(commit_of_tag)
    }

    /// Returns the commit of the given tag.
    #[must_use]
    pub fn get_commit(&self, tag_name: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|(_, tag)| tag.name == tag_name)
            .map(|(commit, _)| commit.as_str())
    }

    /// Returns `true` if the given tag exists.
    #[must_use]
    pub fn contains_commit(&self, commit: &str) -> bool {
        self.tags.contains_key(commit)
    }

    /// Inserts a new tagged commit.
    pub fn insert(&mut self, commit: String, tag: Tag) {
        if let Some(index) = self.commits.get_index_of(&commit)
            && let Err(idx) = self.binary_search(index)
        {
            self.tag_indexes.insert(idx, index);
        }
        self.tags.insert(commit, tag);
    }

    /// Retains only the tags specified by the predicate.
    pub fn retain(&mut self, mut f: impl FnMut(&Tag) -> bool) {
        self.tags.retain(|_, tag| f(tag));
        self.tag_indexes.retain(|&idx| {
            self.commits
                .get_index(idx)
                .is_some_and(|(commit, _)| self.tags.contains_key(commit))
        });
    }

    fn binary_search(&self, index: usize) -> std::result::Result<usize, usize> {
        self.tag_indexes
            .binary_search_by_key(&Reverse(index), |tag_idx| Reverse(*tag_idx))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    use temp_dir::TempDir;

    use super::*;

    fn run_git(path: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .expect("failed to execute git");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("git output should be valid UTF-8")
    }

    fn create_commit(path: &Path, name: &str, content: &str, second: u8) -> String {
        fs::write(path.join(name), content).expect("failed to write test file");
        run_git(path, &["add", "."]);
        let date = format!("2024-01-01T00:00:{second:02}Z");
        let output = Command::new("git")
            .args(["commit", "--no-gpg-sign", "-m", name])
            .env("GIT_AUTHOR_DATE", &date)
            .env("GIT_COMMITTER_DATE", date)
            .current_dir(path)
            .output()
            .expect("failed to execute git commit");
        assert!(
            output.status.success(),
            "git commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        run_git(path, &["rev-parse", "HEAD"])
            .trim_ascii_end()
            .to_string()
    }

    fn create_tagged_repository() -> Result<(Repository, TempDir, Vec<String>)> {
        let temp_dir = TempDir::with_prefix("git-cliff-").expect("failed to create temp dir");
        let path = temp_dir.path();
        run_git(path, &["init"]);
        run_git(path, &["config", "user.email", "test@example.com"]);
        run_git(path, &["config", "user.name", "test"]);

        let commits = vec![
            create_commit(path, "one", "one", 1),
            create_commit(path, "two", "two", 2),
            create_commit(path, "three", "three", 3),
            create_commit(path, "four", "four", 4),
            create_commit(path, "five", "five", 5),
        ];
        run_git(path, &["tag", "v1.0.0", &commits[0]]);
        run_git(path, &["tag", "v2.0.0", &commits[3]]);

        Ok((Repository::discover(path.to_path_buf())?, temp_dir, commits))
    }

    #[test]
    fn gets_closest_tag_for_untagged_commit() -> Result<()> {
        let (repository, _temp_dir, commits) = create_tagged_repository()?;
        let tags = repository.tags(&None, false, false)?;

        assert_eq!(
            tags.get(&commits[0]).expect("expected exact tag").name,
            "v1.0.0"
        );
        assert_eq!(
            tags.get_closest(&commits[1])
                .expect("expected closest tag")
                .name,
            "v2.0.0"
        );
        assert!(tags.get_closest(&commits[4]).is_none());
        Ok(())
    }

    #[test]
    fn retain_updates_closest_tag_indexes() -> Result<()> {
        let (repository, _temp_dir, commits) = create_tagged_repository()?;
        let mut tags = repository.tags(&None, false, false)?;

        tags.retain(|tag| tag.name != "v2.0.0");

        assert!(tags.get(&commits[3]).is_none());
        assert_eq!(
            tags.get_closest(&commits[0])
                .expect("expected retained tag")
                .name,
            "v1.0.0"
        );
        assert!(tags.get_closest(&commits[1]).is_none());
        Ok(())
    }

    #[test]
    fn insert_preserves_closest_tag_order() -> Result<()> {
        let (repository, _temp_dir, commits) = create_tagged_repository()?;
        let mut tags = repository.tags(&None, false, false)?;

        tags.insert(
            commits[2].clone(),
            Tag {
                name: String::from("v1.5.0"),
                message: None,
            },
        );

        assert_eq!(tags.get_commit("v1.5.0"), Some(commits[2].as_str()));
        assert_eq!(
            tags.get_closest(&commits[1])
                .expect("expected inserted tag")
                .name,
            "v1.5.0"
        );
        Ok(())
    }
}
