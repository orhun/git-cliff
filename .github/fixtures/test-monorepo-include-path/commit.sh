#!/usr/bin/env bash
set -e

git remote add origin https://github.com/orhun/git-cliff
git fetch
mv cliff.toml cliff.toml.bak
git checkout 076feb74b4d8c8634669f57d4e2765c39490d80e
mv cliff.toml.bak cliff.toml
