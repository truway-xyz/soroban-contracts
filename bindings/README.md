# Bindings

Generated TypeScript clients for each contract will live here, one package per contract under
`bindings/typescript/`. They are produced by `make bindings` from the release WASM and must
never be edited by hand.

Not generated yet: bindings are produced once the contracts are redeployed on testnet, so that
each client ships with the matching contract ID.
