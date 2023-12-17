use crate::commit::Commit;
use crate::error::{
	Error,
	Result,
};
#[cfg(feature = "github")]
use crate::github::{
	GitHubCommit,
	GitHubContributor,
	GitHubPullRequest,
	GitHubReleaseMetadata,
};
use next_version::NextVersion;
use semver::Version;
use serde::{
	Deserialize,
	Serialize,
};

/// Representation of a release.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release<'a> {
	/// Release version, git tag.
	pub version:   Option<String>,
	/// Commits made for the release.
	pub commits:   Vec<Commit<'a>>,
	/// Commit ID of the tag.
	#[serde(rename = "commit_id")]
	pub commit_id: Option<String>,
	/// Timestamp of the release in seconds, from epoch.
	pub timestamp: i64,
	/// Previous release.
	pub previous:  Option<Box<Release<'a>>>,
	/// Contributors.
	#[cfg(feature = "github")]
	pub github:    GitHubReleaseMetadata,
}

impl<'a> Release<'a> {
	/// Updates the GitHub metadata that is contained in the release.
	///
	/// This function takes two arguments:
	///
	/// - GitHub commits: needed for associating the Git user with the GitHub
	///   username.
	/// - GitHub pull requests: needed for generating the contributor list for
	///   the release.
	#[cfg(feature = "github")]
	pub fn update_github_metadata(
		&mut self,
		mut github_commits: Vec<GitHubCommit>,
		github_pull_requests: Vec<GitHubPullRequest>,
	) -> Result<()> {
		let mut contributors = std::collections::HashSet::new();
		// retain the commits that are not a part of this release for later on
		// checking the first contributors.
		github_commits.retain(|v| {
			if let Some(commit) =
				self.commits.iter_mut().find(|commit| commit.id == v.sha)
			{
				commit.github.username = v.author.clone().and_then(|v| v.login);
				commit.github.pr_number = github_pull_requests
					.iter()
					.find(|pr| pr.merge_commit_sha == Some(v.sha.clone()))
					.map(|v| v.number);
				contributors.insert(GitHubContributor {
					username:      v.author.clone().and_then(|v| v.login),
					pr_number:     commit.github.pr_number,
					is_first_time: false,
				});
				false
			} else {
				true
			}
		});
		// mark contributors as first-time
		self.github.contributors = contributors
			.into_iter()
			.map(|mut v| {
				v.is_first_time = !github_commits
					.iter()
					.map(|v| v.author.clone().and_then(|v| v.login))
					.any(|login| login == v.username);
				v
			})
			.collect();
		Ok(())
	}

	/// Calculates the next version based on the commits.
	pub fn calculate_next_version(&self) -> Result<String> {
		let version = self
			.previous
			.as_ref()
			.and_then(|release| release.version.clone())
			.ok_or_else(|| Error::PreviousVersionNotFound)?;
		let next_version = Version::parse(version.trim_start_matches('v'))?
			.next(
				self.commits
					.iter()
					.map(|commit| commit.message.to_string())
					.collect::<Vec<String>>(),
			)
			.to_string();
		Ok(next_version)
	}
}

/// Representation of a list of releases.
#[derive(Serialize)]
pub struct Releases<'a> {
	/// Releases.
	pub releases: &'a Vec<Release<'a>>,
}

impl<'a> Releases<'a> {
	/// Returns the list of releases as JSON.
	pub fn as_json(&self) -> Result<String> {
		Ok(serde_json::to_string(self.releases)?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn bump_version() -> Result<()> {
		for (expected_version, commits) in [
			("1.1.0", vec!["feat: add xyz", "fix: fix xyz"]),
			("1.0.1", vec!["fix: add xyz", "fix: aaaaaa"]),
			("2.0.0", vec!["feat!: add xyz", "feat: zzz"]),
		] {
			let release = Release {
				version:   None,
				commits:   commits
					.into_iter()
					.map(|v| Commit::from(v.to_string()))
					.collect(),
				commit_id: None,
				timestamp: 0,
				previous:  Some(Box::new(Release {
					version: Some(String::from("1.0.0")),
					..Default::default()
				})),
				github:    GitHubReleaseMetadata {
					contributors: vec![],
				},
			};
			let next_version = release.calculate_next_version()?;
			assert_eq!(expected_version, next_version);
		}
		Ok(())
	}
}
