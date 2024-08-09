#!/usr/bin/env bash
set -e

GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit --allow-empty -m "Initial commit"
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit --allow-empty --author="testa <testa@address.com>" -m "feat: add feature 1"
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit --allow-empty --author="testa <testa@address.com>" -m "feat: add feature 2"
GIT_COMMITTER_DATE="2022-04-06 01:25:11" git commit --allow-empty --author="testb <testb@address.com>" -m "feat: add feature 3"
GIT_COMMITTER_DATE="2022-04-06 01:25:12" git commit --allow-empty --author="testb <testb@address.com>" -m "feat: add feature 4"
GIT_COMMITTER_DATE="2022-04-06 01:25:13" git commit --allow-empty --author="testc <testc@address.com>" -m "feat: add feature 5"
GIT_COMMITTER_DATE="2022-04-06 01:25:14" git commit --allow-empty --author="testc <testc@address.com>" -m "feat: add feature 6"    
