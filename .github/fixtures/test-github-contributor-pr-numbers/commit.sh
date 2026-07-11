#!/usr/bin/env bash
set -e

mv cliff.toml cliff.toml.bak
git remote add origin https://github.com/orhun/git-cliff
git pull origin main
git fetch --tags
mv cliff.toml.bak cliff.toml
