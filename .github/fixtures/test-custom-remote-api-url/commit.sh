#!/usr/bin/env bash
set -e

git remote add origin https://gitlab.archlinux.org/archlinux/arch-repro-status
git fetch
git checkout 5fe2f324db566756ccaf066fe186100a09a87625
