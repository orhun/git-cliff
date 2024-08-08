use super::{RemoteClient, RemoteCommit, RemotePullRequest};
use crate::config::Remote;
use crate::error::{
	Error,
	Result,
};

pub mod cloud;
pub mod server;

/// Environment variable for overriding the REST API url.
const API_URL_ENV: &str = "BITBUCKET_API_URL";

/// Log message to show while fetching data.
pub const START_FETCHING_MSG: &str = "Retrieving data from Bitbucket...";

/// Log message to show when done fetching.
pub const FINISHED_FETCHING_MSG: &str = "Done fetching Bitbucket data.";

/// Template variables related to this remote.
pub(crate) const TEMPLATE_VARIABLES: &[&str] = &["bitbucket", "commit.bitbucket"];

/// Maximum number of entries to fetch for pull requests.
pub(crate) const MAX_PAGE_PRS: usize = 50;

pub enum BitbucketClient {
	Cloud(cloud::Client),
	Server(server::Client),
}

impl TryFrom<Remote> for BitbucketClient {
	type Error = Error;

	fn try_from(value: Remote) -> Result<Self> {
		let client = super::create_remote_client(&value, "application/json")?;

		Ok(if let Ok(url) = std::env::var(API_URL_ENV) {
			Self::Server((value, client, url).into())
		} else {
			Self::Cloud((value, client).into())
		})
	}
}

impl RemoteClient for BitbucketClient {
	fn api_url(&self) -> String {
		match self {
			Self::Cloud(client) => client.api_url(),
			Self::Server(client) => client.api_url(),
		}
	}

	fn remote(&self) -> crate::config::Remote {
		match self {
			Self::Cloud(client) => client.remote(),
			Self::Server(client) => client.remote(),
		}
	}

	fn client(&self) -> reqwest_middleware::ClientWithMiddleware {
		match self {
			Self::Cloud(client) => client.client(),
			Self::Server(client) => client.client(),
		}
	}
}

impl BitbucketClient {
	/// Fetches the Bitbucket API and returns the commits.
	pub async fn get_commits(&self) -> Result<Vec<Box<dyn RemoteCommit>>> {
		match self {
			BitbucketClient::Cloud(client) => client.get_commits().await,
			BitbucketClient::Server(client) => client.get_commits().await,
		}
	}

	/// Fetches the Bitbucket API and returns the pull requests.
	pub async fn get_pull_requests(
		&self,
	) -> Result<Vec<Box<dyn RemotePullRequest>>> {
		match self {
			BitbucketClient::Cloud(client) => client.get_pull_requests().await,
			BitbucketClient::Server(client) => client.get_pull_requests().await,
		}
	}
}
