use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::{Debug, MAX_PAGE_SIZE, RemoteClient, RemoteCommit, RemotePullRequest};
use crate::config::Remote;
use crate::error::{Error, Result};

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
    /// Constructs the URL for Azure DevOps commits API.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn commits_url(api_url: &str, remote: &Remote, ref_name: Option<&str>, page: i32) -> String {
        let skip = page * MAX_PAGE_SIZE as i32;
        let mut url = format!(
            "{}/{}/_apis/git/repositories/{}/commits?api-version=7.1&$top={}&$skip={}",
            api_url,
            urlencoding::encode(&remote.owner),
            urlencoding::encode(&remote.repo),
            MAX_PAGE_SIZE,
            skip
        );

        if let Some(ref_name) = ref_name {
            url.push_str(&format!(
                "&searchCriteria.itemVersion.versionType=tag&searchCriteria.itemVersion.version={}",
                urlencoding::encode(ref_name)
            ));
        }

        url
    }

    /// Constructs the URL for Azure DevOps pull requests API.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn pull_requests_url(api_url: &str, remote: &Remote, page: i32) -> String {
        let skip = page * MAX_PAGE_SIZE as i32;
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

    /// Fetches the complete list of commits.
    /// This is inefficient for large repositories; consider using
    /// `get_commit_stream` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_commits(&self, ref_name: Option<&str>) -> Result<Vec<Box<dyn RemoteCommit>>> {
        use futures::TryStreamExt;
        self.get_commit_stream(ref_name).try_collect().await
    }

    /// Fetches the complete list of pull requests.
    /// This is inefficient for large repositories; consider using
    /// `get_pull_request_stream` instead.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn get_pull_requests(&self) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        use futures::TryStreamExt;
        self.get_pull_request_stream().try_collect().await
    }

    fn get_commit_stream(
        &self,
        ref_name: Option<&str>,
    ) -> impl Stream<Item = Result<Box<dyn RemoteCommit>>> + '_ {
        let ref_name = ref_name.map(ToString::to_string);
        async_stream! {
            let page_stream = stream::iter(0..)
                .map(|page| {
                    let ref_name = ref_name.clone();
                    async move {
                        let url = Self::commits_url(&self.api_url(), &self.remote(), ref_name.as_deref(), page);
                        self.get_json::<AzureDevOpsCommitsResponse>(&url).await
                    }
                })
                .buffered(10);

            let mut page_stream = Box::pin(page_stream);

            while let Some(page_result) = page_stream.next().await {
                match page_result {
                    Ok(response) => {
                        if response.value.is_empty() {
                            break;
                        }

                        for commit in response.value {
                            yield Ok(Box::new(commit) as Box<dyn RemoteCommit>);
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                        break;
                    }
                }
            }
        }
    }

    fn get_pull_request_stream(
        &self,
    ) -> impl Stream<Item = Result<Box<dyn RemotePullRequest>>> + '_ {
        async_stream! {
            let page_stream = stream::iter(0..)
                .map(|page| async move {
                    let url = Self::pull_requests_url(&self.api_url(), &self.remote(), page);
                    self.get_json::<AzureDevOpsPullRequestsResponse>(&url).await
                })
                .buffered(5);

            let mut page_stream = Box::pin(page_stream);

            while let Some(page_result) = page_stream.next().await {
                match page_result {
                    Ok(response) => {
                        if response.value.is_empty() {
                            break;
                        }

                        for pr in response.value {
                            yield Ok(Box::new(pr) as Box<dyn RemotePullRequest>);
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::config::Remote;
    use crate::remote::RemotePullRequest;

    #[test]
    fn commits_url() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsClient::commits_url("https://dev.azure.com", &remote, None, 0);

        assert_eq!(
            "https://dev.azure.com/myorg%2Fmyproject/_apis/git/repositories/myrepo/commits?api-version=7.1&$top=100&$skip=0",
            url
        );
    }

    #[test]
    fn commits_url_with_tag() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url =
            AzureDevOpsClient::commits_url("https://dev.azure.com", &remote, Some("v1.0.0"), 0);

        assert!(url.contains("searchCriteria.itemVersion.versionType=tag"));
        assert!(url.contains("searchCriteria.itemVersion.version=v1.0.0"));
    }

    #[test]
    fn commits_url_pagination() {
        let remote = Remote {
            owner: String::from("org/proj"),
            repo: String::from("repo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsClient::commits_url("https://dev.azure.com", &remote, None, 2);

        assert!(url.contains("$skip=200"));
        assert!(url.contains("$top=100"));
    }

    #[test]
    fn pull_requests_url() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let url = AzureDevOpsClient::pull_requests_url("https://dev.azure.com", &remote, 0);

        assert!(url.contains("pullrequests"));
        assert!(url.contains("searchCriteria.status=completed"));
        assert!(url.contains("$top=100"));
        assert!(url.contains("$skip=0"));
    }

    #[test]
    fn client_try_from_remote() {
        let remote = Remote {
            owner: String::from("myorg/myproject"),
            repo: String::from("myrepo"),
            token: None,
            is_custom: false,
            api_url: None,
            native_tls: None,
        };

        let client = AzureDevOpsClient::try_from(remote.clone());
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(remote.owner, client.remote().owner);
        assert_eq!(remote.repo, client.remote().repo);
    }

    #[test]
    fn pull_request_with_commit_ref_no_commit_id() {
        let pr = AzureDevOpsPullRequest {
            pull_request_id: 1,
            title: Some(String::from("test")),
            status: String::from("completed"),
            created_by: None,
            last_merge_commit: Some(AzureDevOpsCommitRef { commit_id: None }),
            labels: vec![],
        };

        assert_eq!(None, pr.merge_commit());
    }
}
