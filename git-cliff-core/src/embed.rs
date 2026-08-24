use std::path::{Component, Path};
use std::{fs, str};

use rust_embed::RustEmbed;

use crate::config::Config;
use crate::error::{Error, Result};

/// Default configuration file embedder/extractor.
///
/// Embeds `config/`[`DEFAULT_CONFIG`] into the binary.
///
/// [`DEFAULT_CONFIG`]: crate::DEFAULT_CONFIG
#[derive(Debug, RustEmbed)]
#[folder = "../config/"]
pub struct EmbeddedConfig;

impl EmbeddedConfig {
    /// Extracts the embedded content.
    pub fn get_config() -> Result<String> {
        match Self::get(crate::DEFAULT_CONFIG) {
            Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
            None => Err(Error::EmbeddedError(String::from(
                "Embedded config not found",
            ))),
        }
    }

    /// Parses the extracted content into [`Config`].
    ///
    /// [`Config`]: Config
    pub fn parse() -> Result<Config> {
        Self::get_config()?.parse()
    }
}

/// Built-in configuration file embedder/extractor.
///
/// Embeds the files under `/examples/` into the binary.
#[derive(RustEmbed)]
#[folder = "../examples/"]
pub struct BuiltinConfig;

impl BuiltinConfig {
    /// Normalizes a template name to a file name carrying the `.toml`
    /// extension (appending it when absent).
    fn with_toml_extension(name: &str) -> String {
        if Path::new(name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
        {
            name.to_string()
        } else {
            format!("{name}.toml")
        }
    }

    /// Extracts the embedded content.
    pub fn get_config(name: String) -> Result<String> {
        let name = Self::with_toml_extension(&name);
        let contents = match Self::get(&name) {
            Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
            None => Err(Error::EmbeddedError(format!("config {name} not found"))),
        }?;
        Ok(contents)
    }

    /// Parses the extracted content into [`Config`] along with the name.
    ///
    /// [`Config`]: Config
    pub fn parse(name: String) -> Result<(Config, String)> {
        let parsed = Self::get_config(name.clone())?.parse()?;
        Ok((parsed, name))
    }

    /// Validates a user-defined templates directory.
    pub fn validate_templates_dir(dir: &Path) -> Result<()> {
        if !dir.is_dir() {
            return Err(Error::ArgumentError(format!(
                "templates directory does not exist or is not a directory: {}",
                dir.display()
            )));
        }
        Ok(())
    }

    /// Extracts the template content for `name`, preferring a user-provided
    /// template found in `templates_dir` (if given) over the built-in template
    /// of the same name. The `.toml` extension is optional in `name`.
    pub fn get_config_from(name: String, templates_dir: Option<&Path>) -> Result<String> {
        if let Some(dir) = templates_dir {
            Self::validate_templates_dir(dir)?;
            let file_name = Self::with_toml_extension(&name);
            let mut components = Path::new(&file_name).components();
            if !matches!(components.next(), Some(Component::Normal(_))) ||
                components.next().is_some()
            {
                return Err(Error::ArgumentError(format!(
                    "template name must not contain path components: {name}"
                )));
            }
            let path = dir.join(file_name);
            if path.is_file() {
                return Ok(fs::read_to_string(path)?);
            }
        }
        Self::get_config(name)
    }

    /// Lists the names of the built-in templates, without the `.toml`
    /// extension and in sorted order.
    pub fn list() -> Vec<String> {
        let mut names: Vec<String> = Self::iter()
            .filter_map(|file| {
                let path = Path::new(file.as_ref());
                if path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
                {
                    path.file_stem()
                        .map(|stem| stem.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();
        names.sort();
        names
    }

    /// Lists the names of every available template — built-in templates plus
    /// the `.toml` templates found in `templates_dir` (if given) — without the
    /// `.toml` extension, sorted and deduplicated.
    ///
    /// Returns an error if `templates_dir` is provided but cannot be read
    /// (e.g. it does not exist or is not a directory).
    pub fn list_templates(templates_dir: Option<&Path>) -> Result<Vec<String>> {
        let mut names = Self::list();
        if let Some(dir) = templates_dir {
            Self::validate_templates_dir(dir)?;
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                if path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
                {
                    if let Some(stem) = path.file_stem() {
                        names.push(stem.to_string_lossy().to_string());
                    }
                }
            }
        }
        names.sort();
        names.dedup();
        Ok(names)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use temp_dir::TempDir;

    use super::*;

    #[test]
    fn lists_builtin_templates_sorted_without_extension() {
        let names = BuiltinConfig::list();
        // a couple of the shipped example templates are expected to be present
        assert!(
            names.contains(&"github".to_string()),
            "expected built-in 'github' in {names:?}"
        );
        assert!(
            names.contains(&"keepachangelog".to_string()),
            "expected built-in 'keepachangelog' in {names:?}"
        );
        // names are reported without the `.toml` extension
        assert!(
            names.iter().all(|name| !name.ends_with(".toml")),
            "names should not carry the .toml extension: {names:?}"
        );
        // and are returned in sorted order
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted, "names should be sorted");
    }

    #[test]
    fn list_templates_merges_user_directory() -> Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("my-custom.toml"), "")?;
        fs::write(dir.path().join("notes.txt"), "")?; // not a template, ignored

        let names = BuiltinConfig::list_templates(Some(dir.path()))?;

        assert!(
            names.contains(&"my-custom".to_string()),
            "user template should be listed: {names:?}"
        );
        assert!(
            names.contains(&"github".to_string()),
            "built-in templates should still be listed: {names:?}"
        );
        assert!(
            !names.iter().any(|name| name == "notes"),
            "non-.toml files should be ignored: {names:?}"
        );

        // sorted and deduplicated
        let mut expected = names.clone();
        expected.sort();
        expected.dedup();
        assert_eq!(names, expected, "names should be sorted and deduped");
        Ok(())
    }

    #[test]
    fn list_templates_without_user_directory_matches_builtin() -> Result<()> {
        assert_eq!(BuiltinConfig::list_templates(None)?, BuiltinConfig::list());
        Ok(())
    }

    #[test]
    fn list_templates_errors_clearly_when_directory_missing() {
        let dir = TempDir::new().expect("temp dir");
        let missing = dir.path().join("does-not-exist");
        let err = BuiltinConfig::list_templates(Some(&missing))
            .expect_err("a missing templates directory should be an error");
        let message = err.to_string();
        assert!(
            message.contains(&missing.display().to_string()),
            "error should name the offending directory, got: {message}"
        );
    }

    #[test]
    fn get_config_from_prefers_user_template_over_builtin() -> Result<()> {
        let dir = TempDir::new()?;
        // shadow the built-in "github" template with custom content
        fs::write(dir.path().join("github.toml"), "# user override\n")?;
        let contents = BuiltinConfig::get_config_from("github".to_string(), Some(dir.path()))?;
        assert_eq!(contents, "# user override\n");
        Ok(())
    }

    #[test]
    fn get_config_from_normalizes_missing_extension() -> Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("mine.toml"), "# mine\n")?;
        // name given without the `.toml` extension still resolves the user file
        let contents = BuiltinConfig::get_config_from("mine".to_string(), Some(dir.path()))?;
        assert_eq!(contents, "# mine\n");
        Ok(())
    }

    #[test]
    fn get_config_from_falls_back_to_builtin() -> Result<()> {
        let dir = TempDir::new()?;
        // user dir has no "github.toml" → falls back to the embedded built-in
        assert_eq!(
            BuiltinConfig::get_config_from("github".to_string(), Some(dir.path()))?,
            BuiltinConfig::get_config("github".to_string())?
        );
        // and with no user dir at all it matches the built-in too
        assert_eq!(
            BuiltinConfig::get_config_from("github".to_string(), None)?,
            BuiltinConfig::get_config("github".to_string())?
        );
        Ok(())
    }

    #[test]
    fn get_config_from_errors_when_directory_missing() {
        let dir = TempDir::new().expect("temp dir");
        let missing = dir.path().join("does-not-exist");
        let err = BuiltinConfig::get_config_from("github".to_string(), Some(&missing))
            .expect_err("a missing templates directory should be an error");
        assert!(err.to_string().contains(&missing.display().to_string()));
    }

    #[test]
    fn get_config_from_rejects_paths_outside_directory() {
        let dir = TempDir::new().expect("temp dir");
        let err = BuiltinConfig::get_config_from("../github".to_string(), Some(dir.path()))
            .expect_err("a template name with path components should be an error");
        assert!(err.to_string().contains("must not contain path components"));
    }
}
