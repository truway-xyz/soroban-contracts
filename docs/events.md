# Events

> **Status: outline.** The per-contract READMEs list each event; this document will be the
> consolidated reference for indexers.

For each event, document:

- contract and function that emits it;
- topics, in order, with their types;
- data fields with their types and units;
- an example decoded from a real transaction;
- guarantees: when it is emitted and when it is not (for example, an idempotent re-anchor
  emits nothing).

| Contract | Event | Topics |
|---|---|---|
| `fund-tracker` | `FundAllocated` | `("fund_allocated", project_id)` |
| `expense-anchor` | `ExpenseAnchored` | `("expense_anchored", project_id)` |
| `sbt-badge` | `BadgeMinted` | `("badge_minted", badge_type)` |
| `sbt-badge` | `BadgeRevoked` | `("badge_revoked",)` |
