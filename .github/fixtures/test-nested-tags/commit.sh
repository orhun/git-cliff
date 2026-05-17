#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "feat: initial feature"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "fix: bug fix"

# Create nested annotated tag: v0.1.0 -> rc-1 (tag object) -> commit
git tag -a rc-1 -m "release candidate"
git tag -a v0.1.0 -m "release" "$(git rev-parse rc-1)"
