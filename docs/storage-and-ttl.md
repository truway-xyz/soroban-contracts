# Storage and TTL

> **Status: outline.**

## Current policy

Defined once in [`packages/common/src/ttl.rs`](../packages/common/src/ttl.rs) and applied by
every contract:

| Constant | Value |
|---|---|
| `BUMP_THRESHOLD` | 30 days (518,400 ledgers) |
| `BUMP_TO` | 120 days (2,073,600 ledgers) |

Every write extends the entry it touches and the contract instance.

## To document

- Storage layout per contract: which keys are instance and which are persistent, and why.
- What happens to records that are never written again, and how to extend or restore them.
- Rent cost estimates per record.
- Rules for changing storage keys in a deployed contract.
