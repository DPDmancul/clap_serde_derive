#! /bin/sh

# Check if there is a difference in the given file for version from last tag to current tag returns true if difference is found.
# If this is the first tag returns false.

this_tag="$(git tag --sort version:refname | tail -n 1)"
last_tag="$(git tag --sort version:refname | tail -n 2 | head -n 1)"

[ "$last_tag" = "" ] || [ "$last_tag" = "$this_tag" ] && exit 0 # no last tag => return true

# find version in diff, return true if found
git diff "$last_tag" "$this_tag" -- "$1" | grep version
