#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2021-01-23 01:23:45" git commit --allow-empty -m "feat: initial commit"

GIT_COMMITTER_DATE="2021-01-23 01:23:46" git commit --allow-empty -m "feat: add feature 1"
git tag v0.1.0
