#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
git submodule add https://github.com/ratatui/ratatui
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -a -m "feat: add ratatui submodule"
cd ratatui
git checkout 9fb054453dba7af8f99c1c3dc658120f64fbd796
cd ..
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit -a -m "feat: ratatui checkout 1"
git tag v0.1.0
cd ratatui
git checkout 912616af48cb9c1c34b8020ef7e7f053bed70d3e
cd ..
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit -a -m "feat: ratatui checkout 2"
cd ratatui
git checkout c10d0f12e85975bc1e8f41eed693c58eca1894eb
cd ..
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit -a -m "feat: ratatui checkout 3"
git tag v0.2.0
GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit --allow-empty -m "test: add tests"
