# Internal review: Soroban contracts

| | |
|---|---|
| **Date** | 2026-06-30 |
| **Fixes** | 2026-09-22 (see [Status](#status-as-of-2026-09-22)) |
| **Scope** | `fund-tracker`, `expense-anchor`, `sbt-badge` |
| **Type** | Internal documentation and code review. Not an independent audit. |

## Summary

All three contracts fulfil their basic role: an immutable record on the ledger. At the time
of the review, `sbt-badge` was the most mature (admin auth, input validation, events,
one-time `initialize`). `fund-tracker` and `expense-anchor` were simpler and showed gaps
between what their documentation claimed and what the code did.

**Trust model:**

| Layer | Source of truth |
|---|---|
| On-chain | Immutable payload attested by the signer (`project_id`, amounts, hashes, badges) |
| Off-chain | Attribution to organizations and users (platform database) |

With a single platform signing key, the `organization` and `submitted_by` fields record the
platform's address, not the foundation's or the end user's. Attribution of "who did what"
depends on the platform. This is recorded as a design decision in
[ADR 0002](../docs/adr/0002-server-attested-records.md).

## Status as of 2026-09-22

Every finding was fixed except A-01 and M-07, which are documented as design decisions. The
starting point was an earlier authorization and TTL fix (2026-08-04) that had not been
merged; it was ported and extended. **Nothing is deployed**: the fixed code has not been
deployed to any network yet.

| ID | Status | What changed |
|---|---|---|
| A-01 | Documented (design decision) | Records stay attested by the platform: `organization` and `submitted_by` hold the admin signer. Attribution to the foundation lives off-chain. Changing it requires each organization to sign. |
| A-02 | Fixed | `initialize` fails with `already_initialized` in all three contracts. |
| A-03 | Fixed | `allocate` and `anchor` require `caller` to be the admin (`not_admin`) as well as its signature. |
| M-04 | Fixed | `fund-tracker` emits `fund_allocated`. All three contracts use `#[contractevent]` (the previous `publish` API is deprecated in SDK 23). |
| M-05 | Fixed | `fund-tracker`: `amount_xlm > 0`. `expense-anchor`: `amount_xlm >= 0` (reports with no spending are anchored with 0) and a 32-byte `receipt_hash`. `sbt-badge`: one active badge per type and organization. |
| M-06 | Fixed | `extend_ttl` on every write (threshold 30 days, extended to 120), for persistent entries and the instance. |
| M-07 | Documented | No interface change. Since `expense-anchor` records are now immutable, a collision fails instead of overwriting. |

Additional changes:

- **Immutable expenses:** `anchor` on an existing `expense_id` with the same data is a no-op
  (platform retries keep working); with different data it fails with
  `expense_already_exists`. It used to overwrite.
- **`upgrade(new_wasm_hash)`** in all three contracts, admin only. It allows fixes without
  changing the contract ID. Trade-off: the admin can change the code, and every upgrade is
  visible on the ledger.
- Tests: 45 (12 + 14 + 19). Panic tests check the exact message.

## Findings (original review, 2026-06-30)

| ID | Severity | Contracts | Description | Original status | Remediation |
|---|---|---|---|---|---|
| A-01 | High | `fund-tracker`, `expense-anchor` | `caller.require_auth()` makes the signer the `organization`/`submitted_by`; the docs implied the foundation's or user's account | Documented | Admin-proxy auth |
| A-02 | High | `fund-tracker`, `expense-anchor` | `initialize` could be called repeatedly, overwriting `Admin`; the docs said one-shot | Documented | Panic `already_initialized` |
| A-03 | High | `fund-tracker`, `expense-anchor` | `DataKey::Admin` was written but never read on writes | Documented | `require_admin` on writes |
| M-04 | Medium | `fund-tracker` | No event on `allocate` (inconsistent with the other contracts) | Documented | `fund_allocated` event |
| M-05 | Medium | All | No validation of `amount_xlm > 0`, 32-byte `receipt_hash`, or badge uniqueness per organization | Documented, tests added | Input validation |
| M-06 | Medium | All | Persistent storage without `extend_ttl`: risk of archival | Documented | Extend TTL on write |
| M-07 | Medium | `fund-tracker`, `expense-anchor` | IDs truncated to 12 characters (`Symbol`): collision risk without a convention | Documented | None |
| L-08 | Low | CI | Workflow used `wasm32-unknown-unknown`; the build requires `wasm32v1-none` and `stellar contract build` | Fixed | None |
| L-09 | Low | All | Comments in Spanish in Rust code | Fixed | None |

### Consistency matrix (original review)

| Capability | `fund-tracker` | `expense-anchor` | `sbt-badge` |
|---|---|---|---|
| One-time `initialize` | No | No | Yes |
| Admin check on writes | No | No | Yes |
| Emits events | No | Yes | Yes |
| Input validation | No | No | Yes (badge types) |
| Organization parameter separate from signer | No | No | Yes |
| Unit tests | 7 | 9 | 16 |

Test counts include characterization tests added during the review.

## Finding details

### A-01: Authorization model

```rust
caller.require_auth();
organization: caller,
```

Whoever signs the transaction is recorded as `organization`/`submitted_by`. The platform signs
with a single key, so the ledger always shows the platform's address.

**Implication:** the record proves that Truway attested an entry, not that a specific
foundation submitted it.

### A-02: `initialize` not one-shot

`sbt-badge` guarded it with `panic!("already_initialized")`. The other two contracts silently
overwrote `Admin` on every call.

### A-03: `Admin` not enforced

`Admin` was stored in instance storage by `initialize`, but `allocate` and `anchor` never
read it. There was no on-chain access control beyond `caller.require_auth()`.

### M-04: No events in `fund-tracker`

Off-chain indexers had to poll `get_allocation` instead of subscribing to events.

### M-05: No input validation

Behavior at the time, confirmed by characterization tests:

- negative or zero `amount_xlm`: accepted;
- `receipt_hash` of any length: accepted;
- several badges of the same type for the same organization: allowed.

### M-06: TTL and archival

Soroban archives persistent entries when their TTL expires. Without `extend_ttl` on writes,
audit records could become inaccessible. Recommended strategy: extend the TTL on every write,
and optionally on critical reads.

### M-07: `Symbol` collisions

Platform convention: the last 12 characters of the UUID. Two UUIDs sharing a suffix would
collide. Mitigation: guarantee unique suffixes, or move to `Bytes`/`String` identifiers.

## Remaining hardening backlog

Done on 2026-09-22: admin checks, `extend_ttl`, events in `fund-tracker`, input validation,
one-time `initialize`, `upgrade()`. SDK already on 23. Pending:

1. **Explicit `organization`/`submitted_by` parameter** (rest of A-01), only if each
   foundation is to sign. Changes the interface and the platform integration.
2. **Cross-contract validation:** check that `project_id` exists in `fund-tracker` before
   anchoring an expense.
3. **Typed errors** (`#[contracterror]`) instead of text panics, so clients receive error codes.
4. **Admin rotation** (`set_admin`) to rotate the key without redeploying.

## Assumptions about the platform integration

Verified on 2026-09-22 against the platform's integration code:

- The platform passes its own address as `caller` in `allocate` and `anchor`, so the admin
  check does not change real behavior while the admin is that same account.
- `receipt_hash` values are always SHA-256 (32 bytes); `allocate` amounts are positive; reports
  may be anchored with amount 0.
- IDs are the last 12 characters of the UUID without hyphens.

Not verified: that the platform database is the only source of truth for attribution.
