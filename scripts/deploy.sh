#!/usr/bin/env bash
# Deploys every contract to a network and records the result in
# deployments/<network>.json.
#
# Usage: scripts/deploy.sh <testnet|mainnet>
#
# Not implemented yet. Planned flow:
#   1. Build release WASM (scripts/build.sh).
#   2. Install each WASM and deploy one instance per contract.
#   3. Call `initialize` with the admin account.
#   4. Write contract_id, wasm_hash, source_commit and deployed_at to the manifest.
#
# Keys are never read from this repository: the admin and deployer identities
# come from the Stellar CLI keystore of the person deploying.
set -euo pipefail

echo "scripts/deploy.sh is not implemented yet. See deployments/README.md." >&2
exit 1
