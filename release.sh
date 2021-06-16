#!/usr/bin/env bash

if [ -n "$1" ]; then
	cargo run > CHANGELOG.md
	git add -A && git commit -m "chore(release): prepare for $1"
	git -c user.name="git-cliff" \
		-c user.email="git-cliff@protonmail.com" \
		-c user.signingkey="1D2D410A741137EBC544826F4A92FA17B6619297" \
		tag -s -a "$1" -m "$(cargo run -- -u -s all)"
fi
