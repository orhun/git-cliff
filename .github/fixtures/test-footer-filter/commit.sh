#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 00:00:00" git commit --allow-empty -m "Initial commit"
GIT_COMMITTER_DATE="2022-04-06 00:10:00" git commit --allow-empty -m "feat: add feature 1"
git tag v0.1.0
GIT_COMMITTER_DATE="2022-04-06 01:00:00" git commit --allow-empty -m "refactor: change feature 1" -m "BREAKING CHANGE: feature 1 is now different"
GIT_COMMITTER_DATE="2022-04-06 01:10:00" git commit --allow-empty -m "chore: upgrade dependencies" -m "changelog: ignore"
git tag v0.2.0
GIT_COMMITTER_DATE="2022-04-06 02:00:00" git commit --allow-empty -m "test: add tests" -m "footer: some more info"
GIT_COMMITTER_DATE="2022-04-06 02:10:00" git commit --allow-empty -m "test: add more tests" -m "changelog: ignore"
