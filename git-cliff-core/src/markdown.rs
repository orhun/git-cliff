use std::path::Path;

use prettify::print as print_prettify_doc;
use prettify_markdown::format_markdown;

use crate::error::{Error, Result};

/// Returns whether the output path uses a Markdown file extension.
pub fn is_markdown_path(path: Option<&Path>) -> bool {
    path.and_then(|output| output.extension())
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

/// Formats Markdown content using the built-in formatter.
pub fn format_markdown_output(content: &str) -> Result<String> {
    let doc = format_markdown(content)
        .map_err(|error| Error::ChangelogError(error.to_string()))?;
    Ok(print_prettify_doc(doc))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn is_markdown_path_checks_extension() {
        assert!(is_markdown_path(Some(Path::new("CHANGELOG.md"))));
        assert!(is_markdown_path(Some(Path::new("notes.MD"))));
        assert!(!is_markdown_path(Some(Path::new("CHANGELOG.txt"))));
        assert!(!is_markdown_path(None));
    }

    #[test]
    fn format_markdown_output_normalizes_spacing() -> Result<()> {
        let formatted = format_markdown_output("# Title\n\n\n\n- item")?;
        assert_eq!("# Title\n\n- item\n", formatted);
        Ok(())
    }
}
