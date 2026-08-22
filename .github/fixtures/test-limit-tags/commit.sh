#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "feat: prod 240601"
git tag prod-240601
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m "feat: prod 240701"
git tag prod-240701
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty -m "feat: prod 240801"
git tag prod-240801
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty -m "feat: prod 240901"
git tag prod-240901
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit --allow-empty -m "feat: prod 241001"
git tag prod-241001
