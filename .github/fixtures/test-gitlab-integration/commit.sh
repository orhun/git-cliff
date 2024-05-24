#!/usr/bin/env bash
set -e

git remote add origin https://gitlab.com/dark0dave/dotfiles
git pull origin main
git fetch --tags
git checkout 3c048a1
