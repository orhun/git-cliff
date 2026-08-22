#!/usr/bin/env bash
set -e

# Regression test for https://github.com/orhun/git-cliff/issues/498
#
# When a feature branch diverges, gets a release tag on the main line, and is
# then cross-merged, the linearized `git log` can list the feature commit
# *before* the tag commit even though it is not an ancestor of the tag. The
# commit must be assigned by graph reachability (it belongs to the release
# whose tag can actually reach it), not by its position in the flat log.
#
# Graph (oldest at the bottom):
#
#   *   merge feature into main   (HEAD)
#   |\
#   | *   merge main into feature
#   | |\
#   | |/
#   |/|
#   * | important fix             (tag: v0.2.0)
#   | * unreleased feature
#   |/
#   * initial commit              (tag: v0.1.0)
#
# "unreleased feature" is NOT an ancestor of v0.2.0, so it must stay unreleased.

GIT_COMMITTER_DATE="2022-04-06 12:00:00" git commit --allow-empty -m "feat: initial commit"
git tag v0.1.0
MAIN=$(git rev-parse --abbrev-ref HEAD)

# Diverge a feature branch from v0.1.0.
git checkout -b feature
GIT_COMMITTER_DATE="2022-04-06 12:00:01" git commit --allow-empty -m "feat: unreleased feature"

# Advance the main line and cut the v0.2.0 release.
git checkout "$MAIN"
GIT_COMMITTER_DATE="2022-04-06 12:00:02" git commit --allow-empty -m "fix: important fix"
git tag v0.2.0

# Cross-merge the main line into the feature branch (e.g. to resolve conflicts).
git checkout feature
GIT_COMMITTER_DATE="2022-04-06 12:00:03" git merge --no-ff "$MAIN" -m "chore: merge main into feature"

# Merge the feature branch back into the main line.
git checkout "$MAIN"
GIT_COMMITTER_DATE="2022-04-06 12:00:04" git merge --no-ff feature -m "chore: merge feature into main"
