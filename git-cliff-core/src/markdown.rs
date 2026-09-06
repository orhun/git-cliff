//! Markdown post-processing for the generated changelog.

use pulldown_cmark::{Options, Parser};
use pulldown_cmark_to_cmark::cmark_with_options;

use crate::error::Result;

/// Normalizes a Markdown string by round-tripping it through a parser and
/// re-emitter.
///
/// This tidies up formatting that is tedious to get right in a Tera template
/// (heading styles, list markers, blank lines between blocks) without the user
/// having to fiddle with `{%-` / `trim` everywhere. The GitHub-flavored
/// extensions git-cliff templates commonly use (tables, strikethrough, task
/// lists, footnotes, and the rest of GFM) are enabled so they survive the
/// round-trip.
///
/// The formatter options are intentionally conservative: it doesn't reflow
/// text or rewrite links, so template output that is already valid Markdown
/// keeps its structure.
pub fn format_markdown(input: &str) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(input, options);
    let mut formatted = String::with_capacity(input.len());
    // Keep `-` as the bullet marker to match git-cliff's default templates and
    // avoid churning existing changelogs from `-` to `*`.
    let format_options = pulldown_cmark_to_cmark::Options {
        list_token: '-',
        ..Default::default()
    };
    cmark_with_options(parser, &mut formatted, format_options)?;

    // `cmark` doesn't emit a trailing newline, but changelogs conventionally
    // end with one. Preserve whatever the input had at the boundary.
    if input.ends_with('\n') && !formatted.ends_with('\n') {
        formatted.push('\n');
    }
    Ok(formatted)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalizes_messy_markdown() -> Result<()> {
        // Valid but sloppy: a setext heading, missing blank line before a
        // heading, and runs of blank lines between blocks.
        let input = "\
Changelog
=========


## 1.0.0
### Features

- first change
- second change



some text
";
        let formatted = format_markdown(input)?;
        let expected = "\
# Changelog

## 1.0.0

### Features

- first change
- second change

some text
";
        assert_eq!(expected, formatted);
        // Formatting is idempotent: a second pass changes nothing.
        assert_eq!(formatted, format_markdown(&formatted)?);
        Ok(())
    }

    #[test]
    fn keeps_clean_markdown_stable() -> Result<()> {
        let input = "\
# Changelog

## 1.0.0

### Features

- add a thing

### Bug Fixes

- fix a thing
";
        assert_eq!(input, format_markdown(input)?);
        Ok(())
    }
}
