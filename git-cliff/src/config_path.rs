//! Resolution of the configuration file path.

use std::path::{Path, PathBuf};

use git_cliff_core::config::Config;

/// Determines which configuration file to use.
///
/// An explicit path is used when it exists. When the given path does not
/// exist, another configuration source is used instead.
///
/// When `--config` is omitted, a project configuration is discovered by
/// searching a starting directory and its ancestors, then the user
/// configuration directory. Discovery starts from `--workdir` when it is
/// given, so that it follows the directory git-cliff was asked to operate on
/// rather than the directory it was invoked from.
pub fn resolve_config_path(
    config: Option<&Path>,
    workdir: Option<&Path>,
    current_dir: &Path,
    user_config: impl FnOnce() -> Option<PathBuf>,
) -> Option<PathBuf> {
    match config {
        Some(path) if path.exists() => Some(path.to_path_buf()),
        Some(_) => user_config(),
        None => workdir
            .unwrap_or(current_dir)
            .ancestors()
            .find_map(Config::retrieve_project_config_path)
            .or_else(user_config),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use git_cliff_core::DEFAULT_CONFIG;
    use git_cliff_core::error::Result;
    use pretty_assertions::assert_eq;
    use temp_dir::TempDir;

    use super::*;

    /// Creates a temporary directory.
    fn temp_dir() -> Result<TempDir> {
        Ok(TempDir::with_prefix("git-cliff-")?)
    }

    /// Creates a project configuration file in the given directory.
    fn write_config(dir: &Path) -> Result<PathBuf> {
        let path = dir.join(DEFAULT_CONFIG);
        fs::write(&path, "[changelog]\n")?;
        Ok(path)
    }

    #[test]
    fn explicit_config_is_used_when_it_exists() -> Result<()> {
        let config_dir = temp_dir()?;
        let current_dir = temp_dir()?;
        // The configuration lives outside the directory discovery would
        // search, so only the explicit branch can return this path.
        let config = write_config(config_dir.path())?;
        assert_eq!(
            Some(config),
            resolve_config_path(
                Some(&config_dir.path().join(DEFAULT_CONFIG)),
                None,
                current_dir.path(),
                || None
            )
        );
        Ok(())
    }

    #[test]
    fn missing_explicit_config_skips_discovery_and_uses_the_user_config() -> Result<()> {
        let dir = temp_dir()?;
        // A discoverable project configuration is present, so this also
        // proves that giving `--config` skips discovery entirely rather
        // than falling through to it.
        write_config(dir.path())?;
        let user_config = dir.path().join("user-cliff.toml");
        assert_eq!(
            Some(user_config.clone()),
            resolve_config_path(
                Some(&dir.path().join("does-not-exist.toml")),
                None,
                dir.path(),
                || Some(user_config.clone())
            )
        );
        Ok(())
    }

    #[test]
    fn config_is_discovered_from_the_working_directory() -> Result<()> {
        let workdir = temp_dir()?;
        let current_dir = temp_dir()?;
        let config = write_config(workdir.path())?;
        // Both directories hold a configuration, so this asserts which one
        // wins rather than merely that something was found.
        write_config(current_dir.path())?;
        assert_eq!(
            Some(config),
            resolve_config_path(None, Some(workdir.path()), current_dir.path(), || None)
        );
        Ok(())
    }

    #[test]
    fn config_is_discovered_from_the_current_directory_without_workdir() -> Result<()> {
        let dir = temp_dir()?;
        let config = write_config(dir.path())?;
        assert_eq!(
            Some(config),
            resolve_config_path(None, None, dir.path(), || None)
        );
        Ok(())
    }

    #[test]
    fn config_is_discovered_from_a_parent_directory() -> Result<()> {
        let dir = temp_dir()?;
        let nested = dir.path().join("nested");
        fs::create_dir(&nested)?;
        let config = write_config(dir.path())?;
        // Discovery walks up from the starting directory, so a configuration
        // in a parent is found from a subdirectory.
        assert_eq!(
            Some(config),
            resolve_config_path(None, None, &nested, || None)
        );
        Ok(())
    }

    #[test]
    fn user_config_is_used_when_no_project_config_is_discovered() -> Result<()> {
        let dir = temp_dir()?;
        let nested = dir.path().join("nested");
        fs::create_dir(&nested)?;
        // Discovery walks to the filesystem root here, so this assumes no
        // configuration file exists above the temporary directory.
        assert_eq!(
            None,
            dir.path()
                .ancestors()
                .find_map(Config::retrieve_project_config_path),
            "a configuration file above the temporary directory defeats this test"
        );
        let user_config = dir.path().join("user-cliff.toml");
        assert_eq!(
            Some(user_config.clone()),
            resolve_config_path(None, Some(&nested), dir.path(), || Some(
                user_config.clone()
            ))
        );
        Ok(())
    }
}
