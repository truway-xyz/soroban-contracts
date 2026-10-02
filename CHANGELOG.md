# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- `fund-tracker`, `expense-anchor` and `sbt-badge`, imported from the Truway platform with the
  fixes from the 2026-06-30 internal review: one-time `initialize`, admin checks on writes,
  input validation, events on every write, TTL extension on every write, immutable expense
  records and in-place `upgrade`.
- `truway-common` crate with the storage TTL policy shared by every contract.
- Cargo workspace with shared package metadata and dependency versions, and a pinned toolchain.
- CI: formatting, lints, tests and release WASM builds.
- Documentation: per-contract references, architecture decision records, audit history and
  deployment manifests.
