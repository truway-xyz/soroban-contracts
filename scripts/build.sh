#!/usr/bin/env bash
# Builds release WASM for every contract and prints each file's size.
set -euo pipefail
cd "$(dirname "$0")/.."

stellar contract build

for wasm in target/wasm32v1-none/release/*.wasm; do
  printf '%-24s %8s bytes\n' "$(basename "$wasm")" "$(wc -c < "$wasm" | tr -d ' ')"
done
