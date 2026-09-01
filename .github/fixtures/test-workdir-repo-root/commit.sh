#!/usr/bin/env bash
set -e

# Regression test for https://github.com/orhun/git-cliff/issues/1369
# Pointing `--workdir` at the repository root must include every commit,
# not scope the changelog to an empty path (which produced empty output).

echo "readme" > README.md
git add README.md
GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit -m "feat: add readme"

mkdir -p src
echo "main" > src/main.rs
git add src/
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -m "fix: add main module"
