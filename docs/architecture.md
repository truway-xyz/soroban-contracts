# Architecture

> **Status: outline.** Sections below list what each part must cover.

## Components

- The three contracts and what each one records.
- The shared crate `truway-common` and what belongs in it.
- The closed-source platform: what it signs and what it stores off-chain.
- Readers: donors, auditors and indexers reading the network directly.

## Trust model

- What a record proves and what it does not (see [ADR 0002](adr/0002-server-attested-records.md)).
- Where attribution to each foundation lives.
- Who holds which key, and what each key can do.

## Life of an expense record

From an approved expense in the platform to a record anyone can verify:

1. The expense is approved and its receipt stored.
2. The platform computes the SHA-256 hash of the receipt.
3. The platform calls `expense-anchor.anchor`.
4. A donor or auditor verifies the receipt against the record.

## Upgrades and key management

- How `upgrade` works and who can call it.
- Planned role separation: cold multisig admin and hot writer.
