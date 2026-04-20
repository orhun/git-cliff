#!/usr/bin/env bash
set -e

mkdir -p website src docs

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
git tag v0.1.0

echo "index" > website/index.html
git add website/
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -m "feat: add website landing page"

echo "main" > src/main.rs
git add src/
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit -m "feat: add main module"

echo "guide" > docs/guide.md
git add docs/
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit -m "fix: update installation guide"

echo "style" > website/style.css
git add website/
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit -m "fix: fix website styles"

echo "lib" > src/lib.rs
git add src/
GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit -m "feat: add library entrypoint"
