use crate::commit::Commit;
use crate::error::Result;
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
}

impl<'a> Release<'a> {
	/// Calculates the next version based on the commits.
	pub fn calculate_next_version(&self) -> Result<String> {
		match self
			.previous
			.as_ref()
			.and_then(|release| release.version.clone())
		{
			Some(version) => {
				let mut semver = Version::parse(&version);
				let mut prefix = None;
				if semver.is_err() && version.split('.').count() >= 2 {
					let mut found_numeric = false;
					for (i, c) in version.chars().enumerate() {
						if c.is_numeric() && !found_numeric {
							found_numeric = true;
							let version_prefix = version[..i].to_string();
							let remaining = version[i..].to_string();
							let version = Version::parse(&remaining);
							if version.is_ok() {
								semver = version;
								prefix = Some(version_prefix);
								break;
							}
						} else if !c.is_numeric() && found_numeric {
							found_numeric = false;
						}
					}
				}
				let next_version = semver?
					.next(
						self.commits
							.iter()
							.map(|commit| commit.message.trim_end().to_string())
							.collect::<Vec<String>>(),
					)
					.to_string();
				if let Some(prefix) = prefix {
					Ok(format!("{prefix}{next_version}"))
				} else {
					Ok(next_version)
				}
			}
			None => {
				warn!("No releases found, using 0.0.1 as the next version.");
				Ok(String::from("0.0.1"))
			}
		}
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
		for (version, expected_version, commits) in [
			("1.0.0", "1.1.0", vec!["feat: add xyz", "fix: fix xyz"]),
			("1.0.0", "1.0.1", vec!["fix: add xyz", "fix: aaaaaa"]),
			("1.0.0", "2.0.0", vec!["feat!: add xyz", "feat: zzz"]),
			("1.0.0", "2.0.0", vec!["feat!: add xyz\n", "feat: zzz\n"]),
			("2.0.0", "2.0.1", vec!["fix: something"]),
			("foo/1.0.0", "foo/1.1.0", vec![
				"feat: add xyz",
				"fix: fix xyz",
			]),
			("bar/1.0.0", "bar/2.0.0", vec![
				"fix: add xyz",
				"fix!: aaaaaa",
			]),
			("zzz-123/test/1.0.0", "zzz-123/test/1.0.1", vec![
				"fix: aaaaaa",
			]),
			("v100.0.0", "v101.0.0", vec!["feat!: something"]),
			("v1.0.0-alpha.1", "v1.0.0-alpha.2", vec!["fix: minor"]),
			("testing/v1.0.0-beta.1", "testing/v1.0.0-beta.2", vec![
				"feat: nice",
			]),
			("tauri-v1.5.4", "tauri-v1.6.0", vec!["feat: something"]),
			(
				"rocket/rocket-v4.0.0-rc.1",
				"rocket/rocket-v4.0.0-rc.2",
				vec!["chore!: wow"],
			),
			(
				"aaa#/@#$@9384!#%^#@#@!#!239432413-idk-9999.2200.5932-alpha.419",
				"aaa#/@#$@9384!#%^#@#@!#!239432413-idk-9999.2200.5932-alpha.420",
				vec!["feat: damn this is working"],
			),
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
					version: Some(String::from(version)),
					..Default::default()
				})),
			};
			let next_version = release.calculate_next_version()?;
			assert_eq!(expected_version, next_version);
		}
		let empty_release = Release {
			previous: Some(Box::new(Release {
				version: None,
				..Default::default()
			})),
			..Default::default()
		};
		let next_version = empty_release.calculate_next_version()?;
		assert_eq!("0.0.1", next_version);
		Ok(())
	}
}
