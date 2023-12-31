#!/usr/bin/env bash
set -e

git remote add origin https://github.com/orhun/git-cliff-readme-example
git pull origin master
git fetch --tags
{
    echo "06412ac1dd4071006c465dde6597a21d4367a158"
    echo "81fbc6365484abf0b4f4b05d384175763ad8db44"
    echo "e4fd3cf8e2e6f49c0b57f66416e886c37cbb3715"
} >>.cliffignore
