#!/usr/bin/env bash
set -e

current_dir="$(pwd)"
submodule_one_dir="$(mktemp -d)"
submodule_two_dir="$(mktemp -d)"

cd $submodule_one_dir && git init >&2
GIT_COMMITTER_DATE="2022-04-05 01:00:8" git commit --allow-empty -m "feat: submodule_one initial commit"

cd $submodule_two_dir && git init >&2
GIT_COMMITTER_DATE="2022-04-05 01:00:12" git commit --allow-empty -m "feat: submodule_two feature B"

cd $current_dir
GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"

git -c protocol.file.allow=always submodule add $submodule_one_dir submodule_one
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -a -m "feat: add submodule_one"

cd submodule_one
GIT_COMMITTER_DATE="2022-04-05 01:00:10" git commit --allow-empty -m "feat: submodule_one feature A"
GIT_COMMITTER_DATE="2022-04-05 01:00:11" git commit --allow-empty -m "fix: submodule_one fix A"
cd ..

GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit -a -m "feat: submodule_one update 1"
git tag v0.1.0

git -c protocol.file.allow=always submodule add $submodule_two_dir submodule_two
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit -a -m "feat: submodule_two with initial commits"

cd submodule_two
GIT_COMMITTER_DATE="2022-04-05 01:00:13" git commit --allow-empty -m "fix: submodule_two fix B"
cd ..

GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit -a -m "feat: submodule_two update 1"
git tag v0.2.0

cd submodule_two
GIT_COMMITTER_DATE="2022-04-05 01:00:14" git commit --allow-empty -m "fix: submodule_two fix C"
cd ..

GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit -a -m "feat: submodule_two update 2"
