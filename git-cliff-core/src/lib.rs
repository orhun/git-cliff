//! A highly customizable changelog generator ⛰️
//!
//! The crate provides a set of optional features that can be enabled in your
//! `Cargo.toml` file.
//!
//! ## Features
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
/// Commit processing pipeline.
pub mod process;
/// Common release type.
pub mod release;
/// Remote handler.
#[cfg(feature = "remote")]
#[allow(async_fn_in_trait)]
pub mod remote;
/// Git repository.
#[cfg(feature = "repo")]
pub mod repo;
/// Release statistics.
pub mod statistics;
/// Changelog commit processing summary.
pub mod summary;
/// Git tag.
pub mod tag;
/// Template engine.
pub mod template;

/// Default configuration file.
///
/// This is used for the user stored global configuration and embedded
/// configuration.
pub const DEFAULT_CONFIG: &str = "cliff.toml";
/// List of possible configuration file location.
///
/// This list of files is used for finding the
/// configuration file relative to the project directory. The first file has the highest priority.
pub const CONFIG_FILES: &[&str] = &["cliff.toml", ".cliff.toml", ".config/cliff.toml"];
/// Default output file.
pub const DEFAULT_OUTPUT: &str = "CHANGELOG.md";
/// Default ignore file.
pub const IGNORE_FILE: &str = ".cliffignore";

/// Sets a human-readable message on the current progress bar span.
/// This macro only has effect if the `tracing-indicatif` feature is enabled.
#[doc(hidden)]
#[macro_export]
macro_rules! set_progress_message {
    ($($arg:tt)*) => {{
        #[cfg(feature = "tracing-indicatif")]
        {
            use tracing::Span;
            use tracing_indicatif::span_ext::IndicatifSpanExt;
            let msg = format!($($arg)*);
            Span::current().pb_set_message(&msg);
        }
    }};
}
