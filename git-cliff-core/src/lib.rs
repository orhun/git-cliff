//! A highly customizable changelog generator
#![warn(missing_docs, clippy::unwrap_used)]

/// Export regex crate.
pub use regex;
/// Git commit.
pub mod commit;
/// Config file parser.
pub mod config;
/// Embedded file handler.
pub mod embed;
/// Error handling.
pub mod error;
/// Common release type.
pub mod release;
/// Git repository.
pub mod repo;
/// Template engine.
pub mod template;

/// Default configuration file.
pub const DEFAULT_CONFIG: &str = "cliff.toml";
