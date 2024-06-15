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

# Check if we are running on Windows with MINGW64 and if cygpath is available
if [ -n "$MSYSTEM" ] && [ "$MSYSTEM" = "MINGW64" ]; then
  echo "Running inside MINGW64 trying to convert paths to Windows format."
  if command -v cygpath > /dev/null 2>&1; then
    # Convert the path to Windows format
    SCRIPT_DIR=$(cygpath -w "$SCRIPT_DIR")
    FIXTURES_DIR=$(cygpath -w "$FIXTURES_DIR")
  else
    echo "WARNING: cygpath command not found in the PATH. The script may not work correctly on Windows."
    exit 1
  fi
fi

# Show results
echo -e "\n---Run git-cliff---"
cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- -vv --config "$FIXTURES_DIR/cliff.toml" "${@:2}"
