#!/usr/bin/env bash
set -e

git remote add origin https://github.com/orhun/git-cliff
git pull origin main
git fetch --tags
