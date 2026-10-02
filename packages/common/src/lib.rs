//! Building blocks shared by Truway's Soroban contracts.
//!
//! Anything more than one contract needs lives here, so that every contract
//! applies the same policy. Today that is the storage TTL policy; role-based
//! access control and typed errors are planned to land here as well (see the
//! roadmap in the repository README).
#![no_std]

pub mod ttl;
