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
/// Remote contributor.
pub mod contributor;
/// Embedded file handler.
pub mod embed;
/// Error handling.
pub mod error;
/// Common release type.
pub mod release;
/// Remote handler.
#[cfg(feature = "remote")]
#[allow(async_fn_in_trait)]
pub mod remote;
/// Git repository.
#[cfg(feature = "repo")]
pub mod repo;
/// Git tag.
pub mod tag;
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
