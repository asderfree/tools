#!/bin/bash
BLOB_ID=$1
if [ -z "$BLOB_ID" ]; then
  echo "Usage: $0 <blob-id>"
  exit 1
fi

# find all commit that has this blob-id.
COMMITS=$(git log --all --pretty=format:%H --diff-filter=A --find-object="$BLOB_ID")

# reverse every commit, find the file name.
for COMMIT in $COMMITS; do
  git ls-tree -r $COMMIT | grep $BLOB_ID | while read -r LINE; do
    # show the file name
    echo $LINE | awk '{print $4}'
  done
done

