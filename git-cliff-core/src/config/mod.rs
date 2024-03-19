/// Provide a command to migrate from old to new configuration.
pub mod migrate;
/// Deprecated Config models for git-cliff.
pub mod models_v1;
/// Current Config models for git-cliff.
pub mod models_v2;
/// Parsing for git-cliff Config.
pub mod parsing;
/// Tests for git-cliff Config.
#[cfg(test)]
pub mod test;
