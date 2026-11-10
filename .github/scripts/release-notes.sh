#!/usr/bin/env bash
# Prints the CHANGELOG.md entry for a version (the lines under its
# "## <version>" heading, up to the next "## " heading).
# Usage: release-notes.sh <version>
set -euo pipefail

version="${1:?usage: release-notes.sh <version>}"
notes=$(awk -v heading="## $version" '
  index($0, heading " ") == 1 || $0 == heading { found = 1; next }
  found && /^## / { exit }
  found
' CHANGELOG.md)

if [ -z "${notes//[[:space:]]/}" ]; then
  echo "::error::CHANGELOG.md has no entry for $version" >&2
  exit 1
fi

printf '%s\n' "$notes"
