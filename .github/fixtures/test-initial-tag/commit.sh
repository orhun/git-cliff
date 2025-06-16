#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit --allow-empty -m "test: add tests1"
GIT_COMMITTER_DATE="2022-04-06 01:25:14" git commit --allow-empty -m "test: add tests2"
GIT_COMMITTER_DATE="2022-04-06 01:25:15" git commit --allow-empty -m "test: add tests3"
GIT_COMMITTER_DATE="2022-04-06 01:25:16" git commit --allow-empty -m "test: add tests4"