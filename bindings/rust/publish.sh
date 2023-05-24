#!/bin/sh

HERE=`dirname $0`
cd "${HERE}"

rm -rf blst
git worktree prune
git worktree add blst

# --allow-dirty because the temporary blst symbolic link is not committed
cargo publish --allow-dirty "$@"
