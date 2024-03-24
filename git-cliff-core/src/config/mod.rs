/// Embedded file handler.
pub mod embed;
/// Provide a command to migrate from old to new configuration.
pub mod migrate;
/// Base confih models for git-cliff.
pub mod models_base;
/// Deprecated Config models for git-cliff.
pub mod models_v1;
/// Current Config models for git-cliff.
pub mod models_v2;
/// Parsing for git-cliff Config.
pub mod parsing;
/// Tests for git-cliff Config.
#[cfg(test)]
pub mod test;

/// Default configuration file.
pub const DEFAULT_CONFIG_FILENAME: &str = "cliff.toml";
/// Default configuration version.
pub const DEFAULT_CONFIG_VERSION: i64 = 2;