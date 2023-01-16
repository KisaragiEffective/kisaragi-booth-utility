#!/bin/bash
set -x
echo "begin."
grep -v '^#' < .misspell | while read -r line
do
    f="$(mktemp)"
    target="$(echo "$line" | awk '{ print $1 }')"
    replacement="$(echo "$line" | awk '{ print $2 }')"
    sed -E "s/$target/$replacement/g" < README.md > "$f" && mv "$f" README.md
done

echo "done."
