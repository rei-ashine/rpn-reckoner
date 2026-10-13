#!/usr/bin/env bash
# Checks that must pass before a tag push is published to crates.io:
# the tag matches the version in Cargo.toml, CHANGELOG.md has an entry
# for that version, and the tagged commit is on the main branch.
set -euo pipefail

tag="${GITHUB_REF_NAME:?GITHUB_REF_NAME is not set}"
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)

if [ -z "$version" ]; then
  echo "::error::could not read the version from Cargo.toml"
  exit 1
fi

if [ "$tag" != "v$version" ]; then
  echo "::error::tag $tag does not match the version in Cargo.toml (v$version)"
  exit 1
fi

if ! grep -Eq "^## ${version//./\\.}( |$)" CHANGELOG.md; then
  echo "::error::CHANGELOG.md has no '## $version' entry"
  exit 1
fi

git fetch --quiet --no-tags origin main:refs/remotes/origin/main
if ! git merge-base --is-ancestor HEAD origin/main; then
  echo "::error::the tagged commit is not on the main branch"
  exit 1
fi

echo "Release checks passed for $tag"
