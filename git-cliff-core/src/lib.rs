//! A highly customizable changelog generator ⛰️
//!
//! The crate provides a set of optional features that can be enabled in your
//! `Cargo.toml` file.
//!
//! ## Features
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs, clippy::unwrap_used)]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/img/git-cliff.png",
	html_favicon_url = "https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/favicon/favicon.ico"
)]

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
/// GitHub client.
#[cfg(feature = "github")]
pub mod github;
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
/// Default output file.
pub const DEFAULT_OUTPUT: &str = "CHANGELOG.md";
/// Default ignore file.
pub const IGNORE_FILE: &str = ".cliffignore";
