#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty -m \
    "feat(web): feature 1, breaking change in footer

Body feature 1

BREAKING CHANGE: breaking change description feature 1
Signed-off-by: user1 <user1@example.com>
Reviewed-by: user2
"

GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty -m \
    "feat(web)!: feature 2, breaking chain in description

Body feature 2

Signed-off-by: user3 <user3@example.com>
"

GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty -m \
    "feat!: feature 3, use default scope = app

Body feature 2

Signed-off-by: user3 <user3@example.com>
"

GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit --allow-empty -m \
    "fix(scope): fix 1, use scope as group

Body fix 1

Fix: #1
"

GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit --allow-empty -m \
    "fix(front-end): fix 2, no footer

Body fix 2
"

GIT_COMMITTER_DATE="2022-04-06 01:25:14" git commit --allow-empty -m \
    "fix(front-end): fix 3 and 4, no body but footer

Fix: #3
Fix: #4
"

git tag v0.1.0
