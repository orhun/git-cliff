#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "feat: add feature 1"
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty -m "fix: fix feature 1"
git tag v0.1.0
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty -m "feat(gui): add feature 2"
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit --allow-empty -m "fix(gui): fix feature 2"
git tag v0.2.0
