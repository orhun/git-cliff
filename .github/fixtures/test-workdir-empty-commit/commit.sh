#!/usr/bin/env bash
set -e

# Regression test for https://github.com/orhun/git-cliff/issues/1369
# A relative `--workdir .` on a repo whose commits have no file changes must
# still list them. Setting --workdir used to add an include-path, which
# filtered out file-less commits and produced an empty changelog.

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "feat: add readme"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "fix: add main module"
