#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "add feature 1"
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty -m "fix: fix feature 1"
git tag v0.1.0
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty -m "feat(gui): add feature 2"
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit --allow-empty -m "fix(gui): fix feature 2"
git tag v0.2.0
GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit --allow-empty -m "test: add tests"
GIT_COMMITTER_DATE="2022-04-06 01:25:14" git commit --allow-empty -m "feat: add release statistics feature(#452)"
GIT_COMMITTER_DATE="2022-04-06 01:25:14" git commit --allow-empty -m "fix: remove duplication (#452)"
GIT_COMMITTER_DATE="2022-04-06 01:25:14" git commit --allow-empty -m "feat: make git short shat available(#1148)"
GIT_COMMITTER_DATE="2022-04-06 01:25:15" git commit --allow-empty -m "fix(parser): ensure URI parsing is RFC3986-compliant"
