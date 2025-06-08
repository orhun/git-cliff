use std::collections::HashMap;

use crate::release::Release;

use chrono::{
	TimeZone,
	Utc,
};
use serde::{
	Deserialize,
	Serialize,
};

/// Aggregated information about how many times a specific link appeared in
/// commit messages.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkCount {
	/// Text of the link.
	pub text:  String,
	/// URL of the link
	pub href:  String,
	/// The number of times this link was referenced.
	pub count: usize,
}

/// Aggregated statistics about commits in the release.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
	/// The total number of commits included in the release.
	pub commit_count:                   usize,
	/// The time span, in days, from the first to the last commit in the
	/// release. Only present if there is more than one commit.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commit_duration_days:           Option<i64>,
	/// The number of commits that follow the Conventional Commits
	/// specification.
	pub conventional_commit_count:      usize,
	/// The number of times each link was referenced in commit messages.
	pub link_counts:                    Vec<LinkCount>,
	/// The total number of links referenced in all commit messages.
	pub total_link_count:               usize,
	/// The number of days since the previous release.
	/// Only present if this is not the first release.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub days_passed_since_last_release: Option<i64>,
}

impl From<&Release<'_>> for Statistics {
	/// Aggregates various statistics from the release data.
	///
	/// This method computes several metrics based on the current release and
	/// its commits:
	///
	/// - Counts the total number of commits.
	/// - Determines the number of days between the first and last commit.
	/// - Counts the number of commits that follow the Conventional Commits
	///   specification.
	/// - Tallies how many times each link appears across all commit messages.
	/// - Calculates the number of days since the previous release, if
	///   available.
	fn from(release: &Release) -> Self {
		let commit_count = release.commits.len();
		let commit_duration_days = if release.commits.len() < 2 {
			trace!(
				"commit_duration_days: insufficient commits to calculate duration \
				 (found {})",
				release.commits.len()
			);
			None
		} else {
			release
				.commits
				.iter()
				.min_by_key(|c| c.committer.timestamp)
				.zip(release.commits.iter().max_by_key(|c| c.committer.timestamp))
				.and_then(|(first, last)| {
					Utc.timestamp_opt(first.committer.timestamp, 0)
						.single()
						.zip(Utc.timestamp_opt(last.committer.timestamp, 0).single())
						.map(|(start, end)| {
							(end.date_naive() - start.date_naive()).num_days()
						})
				})
		};
		let conventional_commit_count =
			release.commits.iter().filter(|c| c.conv.is_some()).count();
		let mut link_counts: Vec<LinkCount> = release
			.commits
			.iter()
			.fold(HashMap::new(), |mut acc, c| {
				for link in &c.links {
					*acc.entry((link.text.clone(), link.href.clone()))
						.or_insert(0) += 1;
				}
				acc
			})
			.into_iter()
			.map(|((text, href), count)| LinkCount { text, href, count })
			.collect();
		link_counts.sort_by(|lhs, rhs| rhs.count.cmp(&lhs.count));
		let total_link_count: usize = link_counts.iter().map(|l| l.count).sum();
		let days_passed_since_last_release = match release.previous.as_ref() {
			Some(prev) => Utc
				.timestamp_opt(release.timestamp, 0)
				.single()
				.zip(Utc.timestamp_opt(prev.timestamp, 0).single())
				.map(|(curr, prev)| {
					(curr.date_naive() - prev.date_naive()).num_days()
				}),
			None => {
				trace!("days_passed_since_last_release: previous release not found");
				None
			}
		};
		Self {
			commit_count,
			commit_duration_days,
			conventional_commit_count,
			link_counts,
			total_link_count,
			days_passed_since_last_release,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::commit::{
		Commit,
		Signature,
	};
	use crate::config::LinkParser;
	use crate::error::Result;
	use crate::release::Release;
	use lazy_regex::Regex;
	use pretty_assertions::assert_eq;
	#[test]
	fn from() -> Result<()> {
		fn find_count(v: &[LinkCount], text: &str, href: &str) -> Option<usize> {
			v.iter()
				.find(|l| l.text == text && l.href == href)
				.map(|l| l.count)
		}
		let link_parsers = vec![
			LinkParser {
				pattern: Regex::new("RFC(\\d+)")?,
				href:    String::from("rfc://$1"),
				text:    None,
			},
			LinkParser {
				pattern: Regex::new("#(\\d+)")?,
				href:    String::from("https://github.com/$1"),
				text:    None,
			},
		];
		let unconventional_commits = vec![
			Commit {
				id: String::from("123123"),
				message: String::from("add feature"),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649201111,
				},
				..Default::default()
			},
			Commit {
				id: String::from("123123"),
				message: String::from("fix feature"),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649201112,
				},
				..Default::default()
			},
			Commit {
				id: String::from("123123"),
				message: String::from("refactor feature"),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649201113,
				},
				..Default::default()
			},
			Commit {
				id: String::from("123123"),
				message: String::from("add docs for RFC456-related feature"),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649201114,
				},
				..Default::default()
			},
		];
		let conventional_commits = vec![
			Commit {
				id: String::from("123123"),
				message: String::from(
					"perf: improve feature performance, fixes #455",
				),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649287515,
				},
				..Default::default()
			},
			Commit {
				id: String::from("123123"),
				message: String::from("style(schema): fix feature schema"),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649287516,
				},
				..Default::default()
			},
			Commit {
				id: String::from("123123"),
				message: String::from(
					"test: add unit tests for RFC456-related feature",
				),
				committer: Signature {
					name:      Some(String::from("John Doe")),
					email:     Some(String::from("john@doe.com")),
					timestamp: 1649287517,
				},
				..Default::default()
			},
		];
		let commits: Vec<Commit> =
			[unconventional_commits.clone(), conventional_commits.clone()]
				.concat()
				.into_iter()
				.map(|c| c.parse_links(&link_parsers))
				.map(|c| c.clone().into_conventional().unwrap_or(c))
				.collect();
		let release = Release {
			commits,
			timestamp: 1649373910,
			previous: Some(Box::new(Release {
				timestamp: 1649201110,
				..Default::default()
			})),
			repository: Some(String::from("/root/repo")),
			..Default::default()
		};

		let statistics = Statistics::from(&release);
		assert_eq!(release.commits.len(), statistics.commit_count);
		assert_eq!(Some(1), statistics.commit_duration_days);
		assert_eq!(
			conventional_commits.len(),
			statistics.conventional_commit_count
		);
		assert_eq!(
			Some(2),
			find_count(&statistics.link_counts, "RFC456", "rfc://456")
		);
		assert_eq!(
			Some(1),
			find_count(&statistics.link_counts, "#455", "https://github.com/455")
		);
		assert_eq!(3, statistics.total_link_count);
		assert_eq!(Some(2), statistics.days_passed_since_last_release);

		let commits = vec![Commit {
			id: String::from("123123"),
			message: String::from("add feature"),
			committer: Signature {
				name:      Some(String::from("John Doe")),
				email:     Some(String::from("john@doe.com")),
				timestamp: 1649201111,
			},
			..Default::default()
		}];
		let release = Release {
			commits,
			timestamp: 1649373910,
			previous: Some(Box::new(Release {
				timestamp: 1649201110,
				..Default::default()
			})),
			repository: Some(String::from("/root/repo")),
			..Default::default()
		};

		let statistics = Statistics::from(&release);
		assert_eq!(None, statistics.commit_duration_days);

		let commits = vec![];
		let release = Release {
			commits,
			timestamp: 1649373910,
			previous: None,
			repository: Some(String::from("/root/repo")),
			..Default::default()
		};

		let statistics = Statistics::from(&release);
		assert_eq!(None, statistics.days_passed_since_last_release);

		Ok(())
	}
}
