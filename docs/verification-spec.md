# Verification spec

> **Status: outline.** Not yet specified.

This document will define how a public verification code, shown to donors in Truway's
reports, resolves to its record on the network, so that anyone can verify an expense without
an account and without trusting Truway's servers.

## To specify

- Format of the verification code and how it maps to `expense_id` and the contract.
- Network, contract IDs and how to read the record (RPC call or indexer).
- How to hash the receipt file and compare it with `receipt_hash`.
- Expected results and error cases (record missing, hash mismatch, archived entry).
- A reference implementation.
