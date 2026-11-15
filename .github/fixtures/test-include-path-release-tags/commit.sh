#!/usr/bin/env bash
set -e

mkdir -p a b

echo "a" > a/file.txt
git add a
GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit -m "feat: add project a"

echo "b" > b/file.txt
git add b
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -m "feat: add project b"

# Both projects are released from a commit that touches neither of them.
echo "release" > README.md
git add README.md
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit -m "chore: prepare release"
git tag a-1.0.0
git tag b-1.0.0

echo "fixed" > a/file.txt
git add a
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit -m "fix: fix project a"

echo "fixed" > b/file.txt
git add b
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit -m "feat: add feature to project b"
