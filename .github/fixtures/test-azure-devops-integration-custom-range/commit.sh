#!/usr/bin/env bash
set -e

git remote add origin https://dev.azure.com/shiftme/gitcliff/_git/git-cliff-readme-example
git pull origin master
git fetch --tags
