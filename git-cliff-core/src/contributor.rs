use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

/// Representation of a remote contributor.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RemoteContributor {
    /// Username.
    pub username: Option<String>,
    /// Title of the pull request.
    pub pr_title: Option<String>,
    /// The pull request that the user created.
    pub pr_number: Option<i64>,
    /// The first pull request the user created in this release (chronological).
    pub pr_number_first: Option<i64>,
    /// All pull request numbers the user created in this release, in merge order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pr_numbers: Vec<i64>,
    /// Labels of the pull request.
    pub pr_labels: Vec<String>,
    /// Whether if the user contributed for the first time.
    pub is_first_time: bool,
}

impl RemoteContributor {
    /// Records an earlier pull request for this contributor within a release.
    pub(crate) fn track_pull_request(&mut self, pr_number: i64) {
        self.pr_numbers.insert(0, pr_number);
        self.pr_number_first = self.pr_numbers.first().copied();
        self.pr_number = self.pr_numbers.last().copied();
    }

    pub(crate) fn from_pull_request(
        username: Option<String>,
        pr_title: Option<String>,
        pr_number: Option<i64>,
        pr_labels: Vec<String>,
    ) -> Self {
        let pr_numbers: Vec<i64> = pr_number.into_iter().collect();
        Self {
            username,
            pr_title,
            pr_number: pr_numbers.last().copied(),
            pr_number_first: pr_numbers.first().copied(),
            pr_numbers,
            pr_labels,
            is_first_time: false,
        }
    }
}

impl Hash for RemoteContributor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pull_request_sets_first_and_latest() {
        let contributor = RemoteContributor::from_pull_request(
            Some(String::from("iamkroot")),
            Some(String::from("feat: defaults")),
            Some(350),
            vec![],
        );
        assert_eq!(contributor.pr_number, Some(350));
        assert_eq!(contributor.pr_number_first, Some(350));
        assert_eq!(contributor.pr_numbers, vec![350]);
    }

    #[test]
    fn track_pull_request_preserves_merge_order() {
        let mut contributor = RemoteContributor::from_pull_request(
            Some(String::from("iamkroot")),
            Some(String::from("feat: defaults")),
            Some(350),
            vec![],
        );
        contributor.track_pull_request(349);
        assert_eq!(contributor.pr_numbers, vec![349, 350]);
        assert_eq!(contributor.pr_number_first, Some(349));
        assert_eq!(contributor.pr_number, Some(350));
    }
}
