#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "feat: initial feature"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "fix: bug fix"

# Create chain of nested tags: v0.1.0-stable -> v0.1.0-rc -> v0.1.0-staging -> commit
git tag -a v0.1.0-staging -m "staging release"
git tag -a v0.1.0-rc -m "rc release" "$(git rev-parse v0.1.0-staging)"
git tag -a v0.1.0-stable -m "stable release" "$(git rev-parse v0.1.0-rc)"
