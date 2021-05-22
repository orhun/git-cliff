use crate::error::Result;

/// Configuration values.
#[derive(
	Default,
	Debug,
	Clone,
	PartialEq,
	serde_derive::Serialize,
	serde_derive::Deserialize,
)]
pub struct Config {
	pub changelog: ChangelogConfig,
}

/// Changelog configuration.
#[derive(
	Default,
	Debug,
	Clone,
	PartialEq,
	serde_derive::Serialize,
	serde_derive::Deserialize,
)]
pub struct ChangelogConfig {
	pub header:        String,
	pub group_parsers: Vec<GroupParser>,
}

#[derive(
	Default,
	Debug,
	Clone,
	PartialEq,
	serde_derive::Serialize,
	serde_derive::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GroupParser {
	pub regex: String,
	pub group: String,
}

impl Config {
	/// Parses the config file and returns the values.
	pub fn parse(file_name: String) -> Result<Config> {
		let mut config = config::Config::default();
		config
			.merge(config::File::with_name(&file_name))?
			.merge(config::Environment::with_prefix(env!("CARGO_PKG_NAME")))?;
		Ok(config.try_into()?)
	}
}
