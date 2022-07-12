#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2021-01-23 01:23:45" git commit --allow-empty -m "feat: add feature 1" -m "footer: test"

GIT_COMMITTER_DATE="2021-01-23 01:23:46" git commit --allow-empty -m "feat: add feature 2" -m "Signed-off-by: bot"
git tag v0.1.0

GIT_COMMITTER_DATE="2021-01-23 01:23:47" git commit --allow-empty -m "fix: fix feature 1" -m "footer1: xyz" -m "footer2: abc"
