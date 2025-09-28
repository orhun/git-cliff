use async_stream::stream as async_stream;
use futures::{Stream, StreamExt, stream};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use super::*;
use crate::config::Remote;
use crate::error::*;

/// Log message to show while fetching data from Bitbucket.
pub const START_FETCHING_MSG: &str = "Retrieving data from Bitbucket...";

/// Log message to show when done fetching from Bitbucket.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching Bitbucket data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["bitbucket", "commit.bitbucket", "commit.remote"];

/// Maximum number of entries to fetch for bitbucket pull requests.
pub(crate) const BITBUCKET_MAX_PAGE_PRS: usize = 50;

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketCommit {
    /// SHA.
    pub hash: String,
    /// Date of the commit
    pub date: String,
    /// Author of the commit.
    pub author: Option<BitbucketCommitAuthor>,
}

impl RemoteCommit for BitbucketCommit {
    fn id(&self) -> String {
        self.hash.clone()
    }

    fn username(&self) -> Option<String> {
        self.author.clone().and_then(|v| v.login)
    }

    fn timestamp(&self) -> Option<i64> {
        Some(self.convert_to_unix_timestamp(self.date.clone().as_str()))
    }
}

/// Bitbucket Pagination Header
///
/// <https://developer.atlassian.com/cloud/bitbucket/rest/intro/#pagination>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketPagination<T> {
    /// Total number of objects in the response.
    pub size: Option<i64>,
    /// Page number of the current results.
    pub page: Option<i64>,
    /// Current number of objects on the existing page.  Globally, the minimum
    /// length is 10 and the maximum is 100.
    pub pagelen: Option<i64>,
    /// Link to the next page if it exists.
    pub next: Option<String>,
    /// Link to the previous page if it exists.
    pub previous: Option<String>,
    /// List of Objects.
    pub values: Vec<T>,
}

/// Author of the commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketCommitAuthor {
    /// Username.
    #[serde(rename = "raw")]
    pub login: Option<String>,
}

/// Label of the pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestLabel {
    /// Name of the label.
    pub name: String,
}

/// Representation of a single pull request's merge commit
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketPullRequestMergeCommit {
    /// SHA of the merge commit.
    pub hash: String,
}

/// Representation of a single pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitbucketPullRequest {
    /// Pull request number.
    pub id: i64,
    /// Pull request title.
    pub title: Option<String>,
    /// Bitbucket Pull Request Merge Commit
    pub merge_commit: BitbucketPullRequestMergeCommit,
    /// Author of Pull Request
    pub author: BitbucketCommitAuthor,
}

impl RemotePullRequest for BitbucketPullRequest {
    fn number(&self) -> i64 {
        self.id
    }

    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn labels(&self) -> Vec<String> {
        vec![]
    }

    fn merge_commit(&self) -> Option<String> {
        Some(self.merge_commit.hash.clone())
    }
}

/// HTTP client for handling Bitbucket REST API requests.
#[derive(Debug, Clone)]
pub struct BitbucketClient {
    /// Remote.
    remote: Remote,
    /// HTTP client.
    client: ClientWithMiddleware,
}

/// Constructs a Bitbucket client from the remote configuration.
impl TryFrom<Remote> for BitbucketClient {
    type Error = Error;
    fn try_from(remote: Remote) -> Result<Self> {
        Ok(Self {
            client: remote.create_client("application/json")?,
            remote,
        })
    }
}

impl RemoteClient for BitbucketClient {
    const API_URL: &'static str = "https://api.bitbucket.org/2.0/repositories";
    const API_URL_ENV: &'static str = "BITBUCKET_API_URL";

    fn remote(&self) -> Remote {
        self.remote.clone()
    }

    fn client(&self) -> ClientWithMiddleware {
        self.client.clone()
    }

    fn commits_stream<'a>(&'a self) -> impl Stream<Item = Result<Box<dyn RemoteCommit>>> + 'a {
        async_stream! {
            let page_stream = stream::iter(0..)
                .map(|page| async move {
                    let commit_page = page + 1;
                    let url = Self::commits_url(&self.api_url(), &self.remote().owner, &self.remote().repo, commit_page);
                    self.get_json::<BitbucketPagination<BitbucketCommit>>(&url).await
                })
                .buffered(10);

            let mut page_stream = Box::pin(page_stream);

            while let Some(page_result) = page_stream.next().await {
                match page_result {
                    Ok(page) => {
                        if page.values.is_empty() {
                            break;
                        }

                        for commit in page.values {
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

    fn pull_requests_stream<'a>(
        &'a self,
    ) -> impl Stream<Item = Result<Box<dyn RemotePullRequest>>> + 'a {
        async_stream! {
            let page_stream = stream::iter(0..)
                .map(|page| async move {
                    let pr_page = page + 1;
                    let url = Self::pull_requests_url(&self.api_url(), &self.remote().owner, &self.remote().repo, pr_page);
                    self.get_json::<BitbucketPagination<BitbucketPullRequest>>(&url).await
                })
                .buffered(5);

            let mut page_stream = Box::pin(page_stream);

            while let Some(page_result) = page_stream.next().await {
                match page_result {
                    Ok(page) => {
                        if page.values.is_empty() {
                            break;
                        }

                        for pr in page.values {
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

impl BitbucketClient {
    /// Constructs the URL for Bitbucket commits API.
    fn commits_url(api_url: &str, owner: &str, repo: &str, page: i32) -> String {
        format!(
            "{}/{}/{}/commits?pagelen={MAX_PAGE_SIZE}&page={page}",
            api_url, owner, repo
        )
    }

    /// Constructs the URL for Bitbucket pull requests API.
    fn pull_requests_url(api_url: &str, owner: &str, repo: &str, page: i32) -> String {
        format!(
            "{}/{}/{}/pullrequests?pagelen={BITBUCKET_MAX_PAGE_PRS}&page={page}&state=MERGED",
            api_url, owner, repo
        )
    }

    /// Fetches the Bitbucket API and returns the commits.
    pub async fn get_commits(&self, _ref_name: Option<&str>) -> Result<Vec<Box<dyn RemoteCommit>>> {
        use futures::TryStreamExt;

        self.commits_stream().try_collect().await
    }

    /// Fetches the Bitbucket API and returns the pull requests.
    pub async fn get_pull_requests(
        &self,
        _ref_name: Option<&str>,
    ) -> Result<Vec<Box<dyn RemotePullRequest>>> {
        use futures::TryStreamExt;

        self.pull_requests_stream().try_collect().await
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::remote::RemoteCommit;

    #[test]
    fn timestamp() {
        let remote_commit = BitbucketCommit {
            hash: String::from("1d244937ee6ceb8e0314a4a201ba93a7a61f2071"),
            author: Some(BitbucketCommitAuthor {
                login: Some(String::from("orhun")),
            }),
            date: String::from("2021-07-18T15:14:39+03:00"),
        };

        assert_eq!(Some(1626610479), remote_commit.timestamp());
    }
}
