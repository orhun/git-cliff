#!/usr/bin/env bash
set -e

SCRIPT_DIR=$(readlink -f "$(dirname "$0")")

if [ -z "$1" ]; then
    echo "Please input a fixture name."
    exit 1
fi

export FIXTURES_DIR="$SCRIPT_DIR/$1"

# Set up a temporary repository
cd "$(mktemp -d)"
git init

# Commit
"$FIXTURES_DIR/commit.sh"

# Show results
echo -e "\n---Run git-cliff---"
cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- --config "$FIXTURES_DIR/cliff.toml" $2
