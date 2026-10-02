#!/usr/bin/env bash
# Generates TypeScript clients for every contract into bindings/typescript/.
#
# Not implemented yet. Planned flow: build release WASM, then run
# `stellar contract bindings typescript` once per contract.
set -euo pipefail

echo "scripts/generate-bindings.sh is not implemented yet. See bindings/README.md." >&2
exit 1
