#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "feat: empty commit without file change"
echo "foo" > foo.txt
git add foo.txt
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -m "feat: commit with file change"
