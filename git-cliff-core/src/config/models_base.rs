use serde::{
	Deserialize,
	Serialize,
};

/// Meta section of the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaConfig {
	/// The version of the config schema.
	pub version: Option<i64>,
}
