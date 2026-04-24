//! Canonical commit-range selection.
//!
//! Produced from CLI + config by `transform_range`; consumed by
//! `execute_range` to emit a git revision-range string.

#[derive(Debug, Clone)]
pub(crate) struct Endpoint {
    pub rev: String,
    pub inclusive: bool,
}

impl Endpoint {
    pub fn inclusive(rev: impl Into<String>) -> Self {
        Self {
            rev: rev.into(),
            inclusive: true,
        }
    }

    pub fn exclusive(rev: impl Into<String>) -> Self {
        Self {
            rev: rev.into(),
            inclusive: false,
        }
    }

    pub fn as_left(&self) -> String {
        if self.inclusive {
            format!("{}^", self.rev)
        } else {
            self.rev.clone()
        }
    }

    pub fn as_right(&self) -> String {
        if self.inclusive {
            self.rev.clone()
        } else {
            format!("{}^", self.rev)
        }
    }
}

/// Canonical commit-range selection. `None` on either side means the
/// executor falls back to the default for that side (first commit on the
/// left, HEAD on the right).
#[derive(Debug, Clone, Default)]
pub(crate) struct CommitRange {
    pub left: Option<Endpoint>,
    pub right: Option<Endpoint>,
}

impl CommitRange {
    pub(crate) fn resolve_with(
        &mut self,
        repository: &git_cliff_core::repo::Repository,
    ) -> git_cliff_core::error::Result<()> {
        for endpoint in self.left.iter_mut().chain(self.right.iter_mut()) {
            let sha = repository.resolve_rev(&endpoint.rev).map_err(|_| {
                git_cliff_core::error::Error::ArgumentError(format!(
                    "could not resolve revision: {}",
                    endpoint.rev
                ))
            })?;
            endpoint.rev = sha;
        }
        if let Some(endpoint) = &self.left {
            if endpoint.inclusive && repository.is_root_commit(&endpoint.rev)? {
                tracing::warn!(
                    "{} is the root commit; left bound falls back to walking from the root",
                    endpoint.rev
                );
                self.left = None;
            }
        }
        Ok(())
    }
}

pub(crate) fn transform_range(
    args: &crate::args::Opt,
    git_config: &git_cliff_core::config::GitConfig,
    tag_names: &[String],
    current_tag: Option<&str>,
) -> git_cliff_core::error::Result<CommitRange> {
    let start_at = args.start_at.as_ref().or(git_config.start_at.as_ref());
    let start_after = args
        .start_after
        .as_ref()
        .or(git_config.start_after.as_ref());
    let end_at = args.end_at.as_ref().or(git_config.end_at.as_ref());
    let end_before = args.end_before.as_ref().or(git_config.end_before.as_ref());
    if start_at.is_some() && start_after.is_some() {
        return Err(git_cliff_core::error::Error::ArgumentError(String::from(
            "`start_at` and `start_after` are mutually exclusive",
        )));
    }
    if end_at.is_some() && end_before.is_some() {
        return Err(git_cliff_core::error::Error::ArgumentError(String::from(
            "`end_at` and `end_before` are mutually exclusive",
        )));
    }
    let has_new_option =
        start_at.is_some() || start_after.is_some() || end_at.is_some() || end_before.is_some();
    let has_legacy_flag = args.unreleased || args.latest || args.current || args.range.is_some();
    if has_new_option && has_legacy_flag {
        return Err(git_cliff_core::error::Error::ArgumentError(String::from(
            "the new range endpoint options cannot be combined with legacy range flags \
             (`--latest`, `--current`, `--unreleased`, positional `A..B`)",
        )));
    }
    if has_new_option {
        let new_left = start_at
            .map(Endpoint::inclusive)
            .or_else(|| start_after.map(Endpoint::exclusive));
        let new_right = end_at
            .map(Endpoint::inclusive)
            .or_else(|| end_before.map(Endpoint::exclusive));
        return Ok(CommitRange {
            left: new_left,
            right: new_right,
        });
    }
    if let Some(range_str) = &args.range {
        let parts: Vec<&str> = range_str.splitn(2, "..").collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Ok(CommitRange {
                left: Some(Endpoint::exclusive(parts[0])),
                right: Some(Endpoint::inclusive(parts[1])),
            });
        }
    }
    if args.unreleased {
        if let Some(last_tag) = tag_names.last() {
            return Ok(CommitRange {
                left: Some(Endpoint::exclusive(last_tag)),
                right: None,
            });
        }
    }
    if args.latest {
        return Ok(match tag_names.len() {
            0 => CommitRange::default(),
            1 => CommitRange {
                left: None,
                right: Some(Endpoint::inclusive(&tag_names[0])),
            },
            n => CommitRange {
                left: Some(Endpoint::exclusive(&tag_names[n - 2])),
                right: Some(Endpoint::inclusive(&tag_names[n - 1])),
            },
        });
    }
    if args.current {
        if tag_names.len() < 2 {
            return Ok(match tag_names.len() {
                0 => CommitRange::default(),
                _ => CommitRange {
                    left: None,
                    right: Some(Endpoint::inclusive(&tag_names[0])),
                },
            });
        }
        let current = current_tag.ok_or_else(|| {
            git_cliff_core::error::Error::ChangelogError(String::from(
                "No tag exists for the current commit",
            ))
        })?;
        let idx = tag_names.iter().position(|t| t == current).ok_or_else(|| {
            git_cliff_core::error::Error::ChangelogError(String::from(
                "No tag exists for the current commit",
            ))
        })?;
        let prev = idx.checked_sub(1).ok_or_else(|| {
            git_cliff_core::error::Error::ChangelogError(String::from(
                "No suitable tags found. Maybe run with '--topo-order'?",
            ))
        })?;
        return Ok(CommitRange {
            left: Some(Endpoint::exclusive(&tag_names[prev])),
            right: Some(Endpoint::inclusive(&tag_names[idx])),
        });
    }
    Ok(CommitRange::default())
}

/// Render a `CommitRange` as a human-readable math interval (e.g.
/// `[v1.0.0, HEAD)`). Used by the `--dry-run` output; not part of any
/// behavioral pipeline.
pub(crate) fn format_interval(range: &CommitRange) -> String {
    let (lb, lv) = match &range.left {
        None => ('[', "first".to_string()),
        Some(e) if e.inclusive => ('[', e.rev.clone()),
        Some(e) => ('(', e.rev.clone()),
    };
    let (rv, rb) = match &range.right {
        None => ("HEAD".to_string(), ']'),
        Some(e) if e.inclusive => (e.rev.clone(), ']'),
        Some(e) => (e.rev.clone(), ')'),
    };
    format!("{lb}{lv}, {rv}{rb}")
}

/// Render a `CommitRange` into a git revision-range string, or `None` when
/// both sides are unbounded (walk everything).
pub(crate) fn execute_range(range: &CommitRange) -> Option<String> {
    match (&range.left, &range.right) {
        (None, None) => None,
        (None, Some(r)) => Some(r.as_right()),
        (Some(l), None) => Some(format!("{}..HEAD", l.as_left())),
        (Some(l), Some(r)) => Some(format!("{}..{}", l.as_left(), r.as_right())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_inclusive_sets_inclusive_true() {
        let e = Endpoint::inclusive("v1.0.0");
        assert_eq!(e.rev, "v1.0.0");
        assert!(e.inclusive);
    }

    #[test]
    fn endpoint_exclusive_sets_inclusive_false() {
        let e = Endpoint::exclusive("v1.0.0");
        assert_eq!(e.rev, "v1.0.0");
        assert!(!e.inclusive);
    }

    #[test]
    fn as_left_inclusive_appends_caret() {
        assert_eq!(Endpoint::inclusive("v1.0.0").as_left(), "v1.0.0^");
    }

    #[test]
    fn as_left_exclusive_is_bare() {
        assert_eq!(Endpoint::exclusive("v1.0.0").as_left(), "v1.0.0");
    }

    #[test]
    fn as_right_inclusive_is_bare() {
        assert_eq!(Endpoint::inclusive("v1.0.0").as_right(), "v1.0.0");
    }

    #[test]
    fn as_right_exclusive_appends_caret() {
        assert_eq!(Endpoint::exclusive("v1.0.0").as_right(), "v1.0.0^");
    }

    #[test]
    fn commit_range_default_is_unbounded_both_sides() {
        let r = CommitRange::default();
        assert!(r.left.is_none());
        assert!(r.right.is_none());
    }

    #[test]
    fn execute_walks_everything_when_both_unbounded() {
        assert_eq!(execute_range(&CommitRange::default()), None);
    }

    #[test]
    fn execute_emits_right_inclusive_bare() {
        let r = CommitRange {
            left: None,
            right: Some(Endpoint::inclusive("B")),
        };
        assert_eq!(execute_range(&r), Some("B".to_string()));
    }

    #[test]
    fn execute_emits_right_exclusive_with_caret() {
        let r = CommitRange {
            left: None,
            right: Some(Endpoint::exclusive("B")),
        };
        assert_eq!(execute_range(&r), Some("B^".to_string()));
    }

    #[test]
    fn execute_emits_left_exclusive_to_head() {
        let r = CommitRange {
            left: Some(Endpoint::exclusive("A")),
            right: None,
        };
        assert_eq!(execute_range(&r), Some("A..HEAD".to_string()));
    }

    #[test]
    fn execute_emits_left_inclusive_to_head() {
        let r = CommitRange {
            left: Some(Endpoint::inclusive("A")),
            right: None,
        };
        assert_eq!(execute_range(&r), Some("A^..HEAD".to_string()));
    }

    #[test]
    fn execute_emits_full_range_for_both_bounded() {
        let cases: [(Endpoint, Endpoint, &str); 4] = [
            (Endpoint::exclusive("A"), Endpoint::inclusive("B"), "A..B"),
            (Endpoint::exclusive("A"), Endpoint::exclusive("B"), "A..B^"),
            (Endpoint::inclusive("A"), Endpoint::inclusive("B"), "A^..B"),
            (Endpoint::inclusive("A"), Endpoint::exclusive("B"), "A^..B^"),
        ];
        for (left, right, expected) in cases {
            let r = CommitRange {
                left: Some(left),
                right: Some(right),
            };
            assert_eq!(execute_range(&r).as_deref(), Some(expected));
        }
    }

    fn default_opt() -> crate::args::Opt {
        <crate::args::Opt as clap::Parser>::try_parse_from(["git-cliff"]).expect("parse")
    }

    #[test]
    fn transform_default_returns_empty_range() {
        let args = default_opt();
        let git_config = git_cliff_core::config::GitConfig::default();
        let tags: Vec<String> = vec![];
        let range = transform_range(&args, &git_config, &tags, None).expect("transform");
        assert!(range.left.is_none());
        assert!(range.right.is_none());
    }

    fn parse_opt(argv: &[&str]) -> crate::args::Opt {
        <crate::args::Opt as clap::Parser>::try_parse_from(argv).expect("parse")
    }

    #[test]
    fn transform_unreleased_is_exclusive_last_tag_to_unbounded_right() {
        let args = parse_opt(&["git-cliff", "--unreleased"]);
        let git_config = git_cliff_core::config::GitConfig::default();
        let tags = vec!["v1.0.0".to_string()];
        let range = transform_range(&args, &git_config, &tags, None).expect("transform");
        let left = range.left.expect("left bound set");
        assert_eq!(left.rev, "v1.0.0");
        assert!(!left.inclusive);
        assert!(range.right.is_none());
    }

    #[test]
    fn transform_latest_with_two_tags_uses_last_two() {
        let args = parse_opt(&["git-cliff", "--latest"]);
        let git_config = git_cliff_core::config::GitConfig::default();
        let tags = vec!["v1.0.0".to_string(), "v1.1.0".to_string()];
        let range = transform_range(&args, &git_config, &tags, None).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "v1.0.0");
        assert!(!left.inclusive);
        let right = range.right.expect("right");
        assert_eq!(right.rev, "v1.1.0");
        assert!(right.inclusive);
    }

    #[test]
    fn transform_latest_with_one_tag_is_unbounded_left_to_inclusive_tag() {
        let args = parse_opt(&["git-cliff", "--latest"]);
        let git_config = git_cliff_core::config::GitConfig::default();
        let tags = vec!["v1.0.0".to_string()];
        let range = transform_range(&args, &git_config, &tags, None).expect("transform");
        assert!(range.left.is_none());
        let right = range.right.expect("right");
        assert_eq!(right.rev, "v1.0.0");
        assert!(right.inclusive);
    }

    #[test]
    fn transform_current_with_two_tags_anchors_on_current_tag() {
        let args = parse_opt(&["git-cliff", "--current"]);
        let git_config = git_cliff_core::config::GitConfig::default();
        let tags = vec![
            "v1.0.0".to_string(),
            "v1.1.0".to_string(),
            "v1.2.0".to_string(),
        ];
        let range = transform_range(&args, &git_config, &tags, Some("v1.1.0")).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "v1.0.0");
        assert!(!left.inclusive);
        let right = range.right.expect("right");
        assert_eq!(right.rev, "v1.1.0");
        assert!(right.inclusive);
    }

    #[test]
    fn transform_bump_alone_is_unrestricted_range() {
        let args = parse_opt(&["git-cliff", "--bump"]);
        let git_config = git_cliff_core::config::GitConfig::default();
        let tags = vec!["v1.0.0".to_string()];
        let range = transform_range(&args, &git_config, &tags, None).expect("transform");
        assert!(range.left.is_none());
        assert!(range.right.is_none());
    }

    #[test]
    fn transform_positional_a_dotdot_b_is_exclusive_a_inclusive_b() {
        let args = parse_opt(&["git-cliff", "v1.0.0..v2.0.0"]);
        let git_config = git_cliff_core::config::GitConfig::default();
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "v1.0.0");
        assert!(!left.inclusive);
        let right = range.right.expect("right");
        assert_eq!(right.rev, "v2.0.0");
        assert!(right.inclusive);
    }

    #[test]
    fn transform_start_at_is_inclusive_left_unbounded_right() {
        let mut args = parse_opt(&["git-cliff"]);
        args.start_at = Some("v1.0.0".to_string());
        let git_config = git_cliff_core::config::GitConfig::default();
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "v1.0.0");
        assert!(left.inclusive);
        assert!(range.right.is_none());
    }

    #[test]
    fn transform_start_after_is_exclusive_left_unbounded_right() {
        let mut args = parse_opt(&["git-cliff"]);
        args.start_after = Some("v1.0.0".to_string());
        let git_config = git_cliff_core::config::GitConfig::default();
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "v1.0.0");
        assert!(!left.inclusive);
        assert!(range.right.is_none());
    }

    #[test]
    fn transform_end_at_is_inclusive_right_unbounded_left() {
        let mut args = parse_opt(&["git-cliff"]);
        args.end_at = Some("v2.0.0".to_string());
        let git_config = git_cliff_core::config::GitConfig::default();
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        assert!(range.left.is_none());
        let right = range.right.expect("right");
        assert_eq!(right.rev, "v2.0.0");
        assert!(right.inclusive);
    }

    #[test]
    fn transform_end_before_is_exclusive_right_unbounded_left() {
        let mut args = parse_opt(&["git-cliff"]);
        args.end_before = Some("v2.0.0".to_string());
        let git_config = git_cliff_core::config::GitConfig::default();
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        assert!(range.left.is_none());
        let right = range.right.expect("right");
        assert_eq!(right.rev, "v2.0.0");
        assert!(!right.inclusive);
    }

    #[test]
    fn transform_cli_start_at_wins_over_config_start_at() {
        let mut args = parse_opt(&["git-cliff"]);
        args.start_at = Some("cli-rev".to_string());
        let mut git_config = git_cliff_core::config::GitConfig::default();
        git_config.start_at = Some("config-rev".to_string());
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "cli-rev");
        assert!(left.inclusive);
    }

    #[test]
    fn transform_config_start_at_applies_when_cli_unset() {
        let args = parse_opt(&["git-cliff"]);
        let mut git_config = git_cliff_core::config::GitConfig::default();
        git_config.start_at = Some("config-rev".to_string());
        let range = transform_range(&args, &git_config, &[], None).expect("transform");
        let left = range.left.expect("left");
        assert_eq!(left.rev, "config-rev");
        assert!(left.inclusive);
    }

    #[test]
    fn transform_rejects_start_at_and_start_after_together() {
        let mut args = parse_opt(&["git-cliff"]);
        args.start_at = Some("A".to_string());
        args.start_after = Some("B".to_string());
        let git_config = git_cliff_core::config::GitConfig::default();
        let err = transform_range(&args, &git_config, &[], None).unwrap_err();
        assert!(
            matches!(err, git_cliff_core::error::Error::ArgumentError(_)),
            "expected ArgumentError, got: {err:?}"
        );
    }

    #[test]
    fn transform_rejects_end_at_and_end_before_together() {
        let mut args = parse_opt(&["git-cliff"]);
        args.end_at = Some("A".to_string());
        args.end_before = Some("B".to_string());
        let git_config = git_cliff_core::config::GitConfig::default();
        let err = transform_range(&args, &git_config, &[], None).unwrap_err();
        assert!(
            matches!(err, git_cliff_core::error::Error::ArgumentError(_)),
            "expected ArgumentError, got: {err:?}"
        );
    }

    #[test]
    fn transform_rejects_legacy_flag_combined_with_new_option() {
        // Spot-check the 4 legacy-flag shapes x 4 new options.
        // Each arm sets one legacy signal, one new-option field.
        let git_config = git_cliff_core::config::GitConfig::default();
        let mut args = parse_opt(&["git-cliff", "--unreleased"]);
        args.start_at = Some("A".to_string());
        let err = transform_range(&args, &git_config, &["v1".to_string()], None).unwrap_err();
        assert!(matches!(
            err,
            git_cliff_core::error::Error::ArgumentError(_)
        ));

        let mut args = parse_opt(&["git-cliff", "--latest"]);
        args.end_at = Some("B".to_string());
        let err = transform_range(
            &args,
            &git_config,
            &["v1".to_string(), "v2".to_string()],
            None,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            git_cliff_core::error::Error::ArgumentError(_)
        ));

        let mut args = parse_opt(&["git-cliff", "--current"]);
        args.start_after = Some("A".to_string());
        let err = transform_range(
            &args,
            &git_config,
            &["v1".to_string(), "v2".to_string()],
            Some("v2"),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            git_cliff_core::error::Error::ArgumentError(_)
        ));

        let mut args = parse_opt(&["git-cliff", "v1..v2"]);
        args.end_before = Some("B".to_string());
        let err = transform_range(&args, &git_config, &[], None).unwrap_err();
        assert!(matches!(
            err,
            git_cliff_core::error::Error::ArgumentError(_)
        ));
    }

    #[test]
    fn transform_rejects_config_new_option_with_legacy_cli_flag() {
        let args = parse_opt(&["git-cliff", "--unreleased"]);
        let mut git_config = git_cliff_core::config::GitConfig::default();
        git_config.start_at = Some("A".to_string());
        let err = transform_range(&args, &git_config, &["v1".to_string()], None).unwrap_err();
        assert!(matches!(
            err,
            git_cliff_core::error::Error::ArgumentError(_)
        ));
    }

    fn test_repo() -> git_cliff_core::repo::Repository {
        git_cliff_core::repo::Repository::discover(std::path::PathBuf::from(env!(
            "CARGO_MANIFEST_DIR"
        )))
        .expect("discover")
    }

    fn root_sha() -> String {
        let out = std::process::Command::new("git")
            .args(["rev-list", "--max-parents=0", "HEAD"])
            .output()
            .expect("git rev-list");
        std::str::from_utf8(&out.stdout)
            .expect("utf-8")
            .trim()
            .to_string()
    }

    #[test]
    fn resolve_with_errors_on_unknown_left_ref() {
        let repo = test_repo();
        let mut range = CommitRange {
            left: Some(Endpoint::exclusive("this-ref-does-not-exist-xyz")),
            right: None,
        };
        let err = range.resolve_with(&repo).unwrap_err();
        assert!(
            matches!(err, git_cliff_core::error::Error::ArgumentError(_)),
            "expected ArgumentError, got: {err:?}"
        );
    }

    #[test]
    fn resolve_with_accepts_valid_refs() {
        let repo = test_repo();
        let mut range = CommitRange {
            left: Some(Endpoint::exclusive("HEAD~1")),
            right: Some(Endpoint::inclusive("HEAD")),
        };
        range.resolve_with(&repo).expect("resolve");
    }

    #[test]
    fn resolve_with_rewrites_inclusive_root_to_unbounded_left() {
        let repo = test_repo();
        let root = root_sha();
        let mut range = CommitRange {
            left: Some(Endpoint::inclusive(&root)),
            right: None,
        };
        range.resolve_with(&repo).expect("resolve");
        assert!(
            range.left.is_none(),
            "inclusive root should downgrade to None (no `root^`)"
        );
    }

    #[test]
    fn format_interval_default_is_closed_first_to_head() {
        assert_eq!(format_interval(&CommitRange::default()), "[first, HEAD]");
    }

    #[test]
    fn format_interval_inclusive_left_uses_square_bracket() {
        let r = CommitRange {
            left: Some(Endpoint::inclusive("v1.0.0")),
            right: None,
        };
        assert_eq!(format_interval(&r), "[v1.0.0, HEAD]");
    }

    #[test]
    fn format_interval_exclusive_left_uses_paren() {
        let r = CommitRange {
            left: Some(Endpoint::exclusive("v1.0.0")),
            right: None,
        };
        assert_eq!(format_interval(&r), "(v1.0.0, HEAD]");
    }

    #[test]
    fn format_interval_inclusive_right_uses_square_bracket() {
        let r = CommitRange {
            left: None,
            right: Some(Endpoint::inclusive("v2.0.0")),
        };
        assert_eq!(format_interval(&r), "[first, v2.0.0]");
    }

    #[test]
    fn format_interval_exclusive_right_uses_paren() {
        let r = CommitRange {
            left: None,
            right: Some(Endpoint::exclusive("v2.0.0")),
        };
        assert_eq!(format_interval(&r), "[first, v2.0.0)");
    }

    #[test]
    fn format_interval_both_sides_bounded() {
        let r = CommitRange {
            left: Some(Endpoint::exclusive("A")),
            right: Some(Endpoint::exclusive("B")),
        };
        assert_eq!(format_interval(&r), "(A, B)");
    }

    #[test]
    fn resolve_with_preserves_exclusive_root() {
        let repo = test_repo();
        let root = root_sha();
        let mut range = CommitRange {
            left: Some(Endpoint::exclusive(&root)),
            right: None,
        };
        range.resolve_with(&repo).expect("resolve");
        let left = range.left.expect("exclusive root preserved");
        assert_eq!(left.rev, root);
        assert!(!left.inclusive);
    }
}
