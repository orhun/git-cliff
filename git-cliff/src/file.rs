//! File handling utilities.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use git_cliff_core::error::Result;
use tempfile::NamedTempFile;

/// Atomically writes new contents to `path` by replacing it with a fully
/// written temporary file.
///
/// If writing or syncing fails before the rename, the original file remains
/// untouched. A process killed before the rename may leave the temporary file
/// behind, but it cannot leave the target partially written.
pub(crate) fn write(
    path: &Path,
    write: impl FnOnce(&mut dyn io::Write) -> Result<()>,
) -> Result<()> {
    // Resolve symlinks so a symlinked changelog continues to point to its
    // target after replacement, as it did with the previous in-place write.
    let target = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let dir = target
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut temp = NamedTempFile::new_in(dir)?;
    if let Ok(metadata) = fs::metadata(&target) {
        temp.as_file().set_permissions(metadata.permissions())?;
    }
    {
        let mut out = io::BufWriter::new(temp.as_file_mut());
        write(&mut out)?;
        out.flush()?;
    }
    temp.as_file().sync_all()?;
    let _ = temp.persist(&target).map_err(io::Error::from)?;

    // Best effort: make the rename itself durable against power loss.
    if let Some(parent) = target.parent()
        && let Ok(dir) = File::open(parent)
    {
        let _ = dir.sync_all();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use temp_dir::TempDir;

    #[test]
    fn failed_write_keeps_original_file_and_cleans_up_temp_file() {
        let dir = TempDir::with_prefix("git-cliff-atomic-write-").unwrap();
        let path = dir.path().join("CHANGELOG.md");
        let original = "# Changelog\n\n## existing notes\n";
        fs::write(&path, original).unwrap();

        let result = write(&path, |out| {
            out.write_all(b"## new entry\n")?;
            Err(io::Error::other("simulated write failure").into())
        });

        assert!(result.is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn successful_write_replaces_file() {
        let dir = TempDir::with_prefix("git-cliff-atomic-write-").unwrap();
        let path = dir.path().join("CHANGELOG.md");
        fs::write(&path, "old\n").unwrap();

        write(&path, |out| {
            out.write_all(b"new\n")?;
            Ok(())
        })
        .unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "new\n");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[cfg(unix)]
    #[test]
    fn replacement_keeps_file_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = TempDir::with_prefix("git-cliff-atomic-write-").unwrap();
        let path = dir.path().join("CHANGELOG.md");
        fs::write(&path, "old\n").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();

        write(&path, |out| {
            out.write_all(b"new\n")?;
            Ok(())
        })
        .unwrap();

        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}
