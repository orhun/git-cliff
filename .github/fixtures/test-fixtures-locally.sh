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

# todo: add windows detection script
# If running on Windows, convert the path to Windows format
# WARNING: make sure you have LF line endings in your scripts (not CRLF)
#          as this can cause issues with 'diff' command
#SCRIPT_DIR=$(cygpath -w "$SCRIPT_DIR")
#FIXTURES_DIR=$(cygpath -w "$FIXTURES_DIR")

# Show results
echo -e "\n---Run git-cliff---"
cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- -vv --config "$FIXTURES_DIR/cliff.toml" "${@:2}"

# Run again showing side-by-side diff
diff --side-by-side --color \
  <(cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- -vv --config "$FIXTURES_DIR/cliff.toml" "${@:2}") \
  "$FIXTURES_DIR/expected.md"