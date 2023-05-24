#!/usr/bin/env bash

HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
BLST_DIR="$HERE/blst"
rm -rf "$BLST_DIR"
mkdir -p "$BLST_DIR"

TOP="$(git rev-parse --show-toplevel)"
cd "$TOP"
git ls-files --recurse-submodules | tar c -T- | tar x -C "$BLST_DIR"

cd "$HERE"
# --allow-dirty because files in the temporary blst are not committed
cargo publish --allow-dirty "$@"
