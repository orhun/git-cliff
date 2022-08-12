#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m \
"feat: add feature 1
feat: add feature 2
fix: fix feature 1"

GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty -m \
"chore: bump deps
style: apply formatting
fix: fix feature 2"

GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty -m \
"test: add initial tests
test: add more tests
test: update assert statements"

git tag v0.1.0
