#!/usr/bin/env bash
set -e

git remote add origin https://github.com/orhun/git-cliff-readme-example
git pull origin master
git fetch --tags

GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit --allow-empty -m "chore: local-only commit"
