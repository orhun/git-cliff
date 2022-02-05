#!/usr/bin/env bash
set -e

git commit --allow-empty -m "feat: add feature 1"
git tag v0.1.0
sleep 1

git commit --allow-empty -m "feat: add feature 2"
git tag v0.2.0
sleep 1

git checkout v0.1.0
git commit --allow-empty -m "feat: fix feature 1"
git tag v0.1.1
sleep 1
