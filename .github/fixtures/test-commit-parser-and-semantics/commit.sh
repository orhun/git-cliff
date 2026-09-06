#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "chore: initial commit"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "fix: patch a bug" -m "BREAKING CHANGE: update the API"
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty -m "feat: remove an obsolete option"
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty -m "feat: add a replacement" -m "BREAKING CHANGE: update the configuration"
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit --allow-empty -m "feat: remove the legacy API" -m "BREAKING CHANGE: drop legacy support"
git tag v0.1.0
