#!/usr/bin/env bash
set -e

# Same fixed history as test-monorepo-include-path: a real git-cliff checkout
# with stable tags (v2.6.1, v2.7.0) and stable commit SHAs, so the resolved
# `--dry-run` revision range is deterministic across runs.
git remote add origin https://github.com/orhun/git-cliff
git fetch
mv cliff.toml cliff.toml.bak
git checkout 076feb74b4d8c8634669f57d4e2765c39490d80e
mv cliff.toml.bak cliff.toml
