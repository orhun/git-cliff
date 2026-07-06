#!/usr/bin/env bash
set -euo pipefail

# Package managers that build from a crate source tarball can't see files
# outside `git-cliff-core/`, so copy the embedded assets into the crate and
# rewrite the embed paths before packaging.
mkdir -p git-cliff-core/config
cp config/cliff.toml git-cliff-core/config/

rm -rf git-cliff-core/examples
cp -r examples git-cliff-core/

sed -i.bak 's|"../config/"|"config/"|' git-cliff-core/src/embed.rs
sed -i.bak 's|"../examples/"|"examples/"|' git-cliff-core/src/embed.rs
rm -f git-cliff-core/src/embed.rs.bak
