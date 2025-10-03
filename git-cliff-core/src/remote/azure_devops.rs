use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::*;
use crate::config::Remote;
use crate::error::*;

/// Log message to show while fetching data from Azure DevOps.
pub const START_FETCHING_MSG: &str = "Retrieving data from Azure DevOps...";

/// Log message to show when done fetching from Azure DevOps.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching Azure DevOps data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &[
    "azure_devops",
    "commit.azure_devops",
    "commit.remote",
    "remote.azure_devops",
];

/// Representation of a single commit.
///
/// <https://learn.microsoft.com/en-us/rest/api/azure/devops/git/commits/get-commits>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsCommit {
    /// Commit ID (SHA-1).
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// Author of the commit.
    pub author: Option<AzureDevOpsCommitAuthor>,
    /// Committer of the commit.
    pub committer: Option<AzureDevOpsCommitAuthor>,
}

impl RemoteCommit for AzureDevOpsCommit {
    fn id(&self) -> String {
        self.commit_id.clone()
    }

    fn username(&self) -> Option<String> {
        self.author.clone().and_then(|v| v.name)
    }

    fn timestamp(&self) -> Option<i64> {
        self.author
            .clone()
            .and_then(|v| v.date)
            .map(|date| self.convert_to_unix_timestamp(&date))
    }
}

/// Azure DevOps commits API response wrapper.
///
/// <https://learn.microsoft.com/en-us/rest/api/azure/devops/git/commits/get-commits>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsCommitsResponse {
    /// List of commits.
    pub value: Vec<AzureDevOpsCommit>,
    /// Number of commits in the response.
    pub count: i64,
}

impl RemoteEntry for AzureDevOpsCommitsResponse {
    fn url(_id: i64, api_url: &str, remote: &Remote, ref_name: Option<&str>, page: i32) -> String {
        let skip = page * MAX_PAGE_SIZE as i32;
        // Azure DevOps format: owner should be "organization/project"
        // and repo is the repository name
        let mut url = format!(
            "{}/{}/_apis/git/repositories/{}/commits?api-version=7.1&$top={}&$skip={}",
            api_url,
            urlencoding::encode(&remote.owner),
            urlencoding::encode(&remote.repo),
            MAX_PAGE_SIZE,
            skip
        );

        if let Some(ref_name) = ref_name {
            // Azure DevOps needs versionType to distinguish between branch/tag/commit
            // For git-cliff, ref_name is typically a tag, but could be a branch or commit
            // We'll default to tag since that's most common with version ranges
            url.push_str(&format!(
                "&searchCriteria.itemVersion.versionType=tag&searchCriteria.itemVersion.version={}",
                urlencoding::encode(ref_name)
            ));
        }

        url
    }

    fn buffer_size() -> usize {
        10
    }

    fn early_exit(&self) -> bool {
        self.value.is_empty()
    }
}

/// Author/Committer of the commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsCommitAuthor {
    /// Name of the author/committer.
    pub name: Option<String>,
    /// Email of the author/committer.
    pub email: Option<String>,
    /// Date of the commit.
    pub date: Option<String>,
}

/// Representation of a single pull request.
///
/// <https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests/get-pull-requests>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsPullRequest {
    /// Pull request ID.
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: i64,
    /// Pull request title.
    pub title: Option<String>,
    /// Status of the pull request.
    pub status: String,
    /// Created by user.
    #[serde(rename = "createdBy")]
    pub created_by: Option<AzureDevOpsUser>,
    /// Last merge commit.
    #[serde(rename = "lastMergeCommit")]
    pub last_merge_commit: Option<AzureDevOpsCommitRef>,
    /// Labels associated with the pull request.
    #[serde(default)]
    pub labels: Vec<AzureDevOpsPullRequestLabel>,
}

impl RemotePullRequest for AzureDevOpsPullRequest {
    fn number(&self) -> i64 {
        self.pull_request_id
    }

    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn labels(&self) -> Vec<String> {
        self.labels.iter().map(|v| v.name.clone()).collect()
    }

    fn merge_commit(&self) -> Option<String> {
        self.last_merge_commit.clone().and_then(|v| v.commit_id)
    }
}

/// Azure DevOps pull requests API response wrapper.
///
/// <https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests/get-pull-requests>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsPullRequestsResponse {
    /// List of pull requests.
    pub value: Vec<AzureDevOpsPullRequest>,
    /// Number of pull requests in the response.
    pub count: i64,
}

impl RemoteEntry for AzureDevOpsPullRequestsResponse {
    fn url(_id: i64, api_url: &str, remote: &Remote, _ref_name: Option<&str>, page: i32) -> String {
        let skip = page * MAX_PAGE_SIZE as i32;
        // Azure DevOps format: owner should be "organization/project"
        // and repo is the repository name
        format!(
            "{}/{}/_apis/git/repositories/{}/pullrequests?api-version=7.1&searchCriteria.\
             status=completed&$top={}&$skip={}",
            api_url,
            urlencoding::encode(&remote.owner),
            urlencoding::encode(&remote.repo),
            MAX_PAGE_SIZE,
            skip
        )
    }

    fn buffer_size() -> usize {
        5
    }

    fn early_exit(&self) -> bool {
        self.value.is_empty()
    }
}

/// Label of the pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsPullRequestLabel {
    /// Name of the label.
    pub name: String,
}

/// Representation of a commit reference.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsCommitRef {
    /// Commit ID (SHA-1).
    #[serde(rename = "commitId")]
    pub commit_id: Option<String>,
}

/// Representation of an Azure DevOps user.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsUser {
    /// Display name of the user.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Unique name of the user.
    #[serde(rename = "uniqueName")]
    pub unique_name: Option<String>,
}

/// HTTP client for handling Azure DevOps REST API requests.
#[derive(Debug, Clone)]
pub struct AzureDevOpsClient {
    /// Remote.
    remote: Remote,
    /// HTTP client.
    client: ClientWithMiddleware,
}

/// Constructs an Azure DevOps client from the remote configuration.
impl TryFrom<Remote> for AzureDevOpsClient {
    type Error = Error;
    fn try_from(remote: Remote) -> Result<Self> {
        Ok(Self {
            client: remote.create_client("application/json")?,
            remote,
        })
    }
}

impl RemoteClient for AzureDevOpsClient {
    const API_URL: &'static str = "https://dev.azure.com";
    const API_URL_ENV: &'static str = "AZURE_DEVOPS_API_URL";

    fn remote(&self) -> Remote {
        self.remote.clone()
    }

    fn client(&self) -> ClientWithMiddleware {
        self.client.clone()
    }
}

impl AzureDevOpsClient {
    /// Fetches the Azure DevOps API and returns the commits.
    pub async fn get_commits(&self, ref_name: Option<&str>) -> Result<Vec<Box<dyn RemoteCommit>>> {
        Ok(self
            .fetch_with_early_exit::<AzureDevOpsCommitsResponse>(0, ref_name)
            .await?
            .into_iter()
            .flat_map(|v| v.value)
            .map(|v| Box::new(v) as Box<dyn RemoteCommit>)
            .collect())
    }

    /// Fetches the Azure DevOps API and returns the pull requests.
    pub async fn get_pull_requests(
        &self,
        ref_name: Option<&str>,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        Ok(self
            .fetch_with_early_exit::<AzureDevOpsPullRequestsResponse>(0, ref_name)
            .await?
            .into_iter()
            .flat_map(|v| v.value)
            .map(|v| Box::new(v) as Box<dyn RemotePullRequest>)
            .collect())
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::config::Remote;
    use crate::remote::{RemoteCommit, RemoteEntry, RemotePullRequest};

    #[test]
    fn timestamp() {
        let remote_commit = AzureDevOpsCommit {
            commit_id: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
            author: Some(AzureDevOpsCommitAuthor {
                name: Some(String::from("orhun")),
                email: Some(String::from("orhun@example.com")),
                date: Some(String::from("2021-07-18T15:14:39+03:00")),
            }),
            committer: None,
        };

        assert_eq!(Some(1626610479), remote_commit.timestamp());
    }

    #[test]
    fn commit_username() {
        let commit = AzureDevOpsCommit {
            commit_id: String::from("abc123"),
            author: Some(AzureDevOpsCommitAuthor {
                name: Some(String::from("test_user")),
                email: Some(String::from("test@example.com")),
                date: Some(String::from("2021-07-18T15:14:39+03:00")),
            }),
            committer: None,
        };

        assert_eq!(Some(String::from("test_user")), commit.username());
        assert_eq!(String::from("abc123"), commit.id());
    }

    #[test]
    fn commit_no_author() {
        let commit = AzureDevOpsCommit {
            commit_id: String::from("abc123"),
            author: None,
            committer: None,
        };

        assert_eq!(None, commit.username());
        assert_eq!(None, commit.timestamp());
    }

    #[test]
    fn pull_request_properties() {
        let pr = AzureDevOpsPullRequest {
            pull_request_id: 42,
            title: Some(String::from("Test PR")),
            status: String::from("completed"),
            created_by: None,
            last_merge_commit: Some(AzureDevOpsCommitRef {
                commit_id: Some(String::from("merge123")),
            }),
            labels: vec![
                AzureDevOpsPullRequestLabel {
                    name: String::from("bug"),
                },
                AzureDevOpsPullRequestLabel {
                    name: String::from("feature"),
                },
            ],
        };

        assert_eq!(42, pr.number());
        assert_eq!(Some(String::from("Test PR")), pr.title());
        assert_eq!(Some(String::from("merge123")), pr.merge_commit());
        assert_eq!(vec![String::from("bug"), String::from("feature")], pr.labels());
    }

    #[test]
    fn pull_request_no_merge_commit() {
        let pr = AzureDevOpsPullRequest {
            pull_request_id: 1,
            title: None,
            status: String::from("active"),
            created_by: None,
            last_merge_commit: None,
            labels: vec![],
        };

        assert_eq!(None, pr.merge_commit());
        assert_eq!(None, pr.title());
        assert!(pr.labels().is_empty());
    }

    #[test]
    fn commits_response_url() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsCommitsResponse::url(
            0,
            "https://dev.azure.com",
            &remote,
            None,
            0,
        );

        assert_eq!(
            "https://dev.azure.com/myorg%2Fmyproject/_apis/git/repositories/myrepo/commits?api-version=7.1&$top=100&$skip=0",
            url
        );
    }

    #[test]
    fn commits_response_url_with_tag() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsCommitsResponse::url(
            0,
            "https://dev.azure.com",
            &remote,
            Some("v1.0.0"),
            0,
        );

        assert!(url.contains("searchCriteria.itemVersion.versionType=tag"));
        assert!(url.contains("searchCriteria.itemVersion.version=v1.0.0"));
    }

    #[test]
    fn commits_response_url_pagination() {
        let remote = Remote {
            owner: String::from("org/proj"),
            repo: String::from("repo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsCommitsResponse::url(
            0,
            "https://dev.azure.com",
            &remote,
            None,
            2,
        );

        assert!(url.contains("$skip=200"));
        assert!(url.contains("$top=100"));
    }

    #[test]
    fn pull_requests_response_url() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsPullRequestsResponse::url(
            0,
            "https://dev.azure.com",
            &remote,
            None,
            0,
        );

        assert!(url.contains("pullrequests"));
        assert!(url.contains("searchCriteria.status=completed"));
        assert!(url.contains("$top=100"));
        assert!(url.contains("$skip=0"));
    }

    #[test]
    fn commits_response_early_exit() {
        let empty_response = AzureDevOpsCommitsResponse {
            value: vec![],
            count: 0,
        };
        assert!(empty_response.early_exit());

        let non_empty_response = AzureDevOpsCommitsResponse {
            value: vec![AzureDevOpsCommit {
                commit_id: String::from("abc"),
                author: None,
                committer: None,
            }],
            count: 1,
        };
        assert!(!non_empty_response.early_exit());
    }

    #[test]
    fn pull_requests_response_early_exit() {
        let empty_response = AzureDevOpsPullRequestsResponse {
            value: vec![],
            count: 0,
        };
        assert!(empty_response.early_exit());

        let non_empty_response = AzureDevOpsPullRequestsResponse {
            value: vec![AzureDevOpsPullRequest {
                pull_request_id: 1,
                title: None,
                status: String::from("completed"),
                created_by: None,
                last_merge_commit: None,
                labels: vec![],
            }],
            count: 1,
        };
        assert!(!non_empty_response.early_exit());
    }

    #[test]
    fn buffer_sizes() {
        assert_eq!(10, AzureDevOpsCommitsResponse::buffer_size());
        assert_eq!(5, AzureDevOpsPullRequestsResponse::buffer_size());
    }
}
