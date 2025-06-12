#!/usr/bin/env bash
set -e

git remote add origin https://github.com/mta-solutions/fsharp-data-validation
git pull origin main
git fetch --tags
git checkout 9201e2729ad3afb34171c493c2cb9984e9d64784