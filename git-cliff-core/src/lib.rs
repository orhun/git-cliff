//! A highly customizable changelog generator
//!
//! ## Features
//!
//! The [cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
//! of the library are:
//! - `repo`: Enable parsing commits from a git repository. Enabled by default.
//!   You can turn this off if you already have the commits to put in the
//!   changelog and you don't need `git-cliff` to parse them.
#![warn(missing_docs, clippy::unwrap_used)]

/// Changelog generator.
pub mod changelog;
/// Command runner.
pub mod command;
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
#[cfg(feature = "repo")]
/// Git repository.
pub mod repo;
/// Template engine.
pub mod template;

#[macro_use]
extern crate log;

/// Default configuration file.
pub const DEFAULT_CONFIG: &str = "cliff.toml";
