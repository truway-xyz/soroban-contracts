# 1. Record architecture decisions

- **Status:** Accepted
- **Date:** 2026-10-01

## Context

These contracts hold public records that cannot be edited once written, and their interfaces
are consumed by the platform, by generated bindings and by anyone who verifies a record.
Decisions about interfaces, storage layout, authorization and upgrades have long-lived
consequences, and the reasoning behind them is easy to lose.

## Decision

We record every significant design decision as an Architecture Decision Record in
`docs/adr/`, using [`template.md`](template.md). ADRs are numbered sequentially and never
deleted: a decision that changes gets a new ADR that supersedes the old one.

## Consequences

- Reviewers and auditors can see why the contracts work the way they do.
- Changes to interfaces or storage need an ADR in the same pull request.
