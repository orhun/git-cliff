#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2021-01-23 01:23:45" git commit --allow-empty -m "feat(bar): add feature A to bar"
GIT_COMMITTER_DATE="2021-01-23 01:23:45" git commit --allow-empty -m "feat(foo): add feature B to foo"
git tag v0.1.0

GIT_COMMITTER_DATE="2021-01-23 01:23:45" git commit --allow-empty -m "fix(foo): fix feature B in foo"
GIT_COMMITTER_DATE="2021-01-23 01:23:45" git commit --allow-empty -m "fix(bar): fix feature A in bar"
git tag v0.2.0
