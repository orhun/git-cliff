#!/usr/bin/env bash

# takes the tag as an argument (e.g. v0.1.0)
if [ -n "$1" ]; then
	# update the version
	msg="# managed by release.sh"
	sed -E -i "s/^version = .* $msg$/version = \"${1#v}\" $msg/" git-cliff*/Cargo.toml
	sed -E -i "s/\"version\": \".+\"/\"version\": \"${1#v}\"/" npm/git-cliff/package.json
	sed -E -i "s/\"(git-cliff-.+)\": \".+\"/\"\1\": \"${1#v}\"/g" npm/git-cliff/package.json
	# update the changelog
	sed -E -i "s/\s+\#\s(.*)\s\#\sreplace issue numbers/\\t\1/g" config/cliff.toml
	cargo run -- --tag "$1" > CHANGELOG.md
	git restore config/cliff.toml
	git add -A && git commit -m "chore(release): prepare for $1"
	git show
	# generate a changelog for the tag message
	export GIT_CLIFF_TEMPLATE="\
	{% for group, commits in commits | group_by(attribute=\"group\") %}
	{{ group | upper_first }}\
	{% for commit in commits %}
		- {% if commit.breaking %}(breaking) {% endif %}{{ commit.message | upper_first }} ({{ commit.id | truncate(length=7, end=\"\") }})\
	{% endfor %}
	{% endfor %}"
	changelog=$(cargo run -- --config examples/detailed.toml --unreleased --strip all)
	# create a signed tag
	# https://keyserver.ubuntu.com/pks/lookup?search=0x4A92FA17B6619297&op=vindex
	git -c user.name="git-cliff" \
		-c user.email="git-cliff@protonmail.com" \
		-c user.signingkey="1D2D410A741137EBC544826F4A92FA17B6619297" \
		tag -s -a "$1" -m "Release $1" -m "$changelog"
	git tag -v "$1"
else
	echo "warn: please provide a tag"
fi
