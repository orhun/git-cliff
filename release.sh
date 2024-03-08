#!/usr/bin/env bash

if ! command -v typos &>/dev/null; then
  echo "typos is not installed. Run 'cargo install typos-cli' to install it, otherwise the typos won't be fixed"
fi

if [ -z "$1" ]; then
	echo "Please provide a tag."
	echo "Usage: ./release.sh v[X.Y.Z]"
	exit
fi

echo "Preparing $1..."
# update the version
msg="# managed by release.sh"
sed -E -i "s/^version = .* $msg$/version = \"${1#v}\" $msg/" git-cliff*/Cargo.toml
sed -E -i "s/\"version\": \".+\"/\"version\": \"${1#v}\"/" npm/git-cliff/package.json
sed -E -i "s/\"(git-cliff-.+)\": \".+\"/\"\1\": \"${1#v}\"/g" npm/git-cliff/package.json
# update the changelog
cargo run -- --config cliff.toml --tag "$1" >CHANGELOG.md
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
echo "Done!"
echo "Now push the commit (git push) and the tag (git push --tags)."
