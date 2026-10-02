//! # sbt-badge
//!
//! Non-transferable (soulbound) reputation badges for organizations. Revoked
//! badges stay on the ledger with `status = Revoked`. See the crate README for
//! the interface, storage layout and events.
#![no_std]

mod contract;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use contract::{SbtBadge, SbtBadgeClient};
pub use events::{BadgeMinted, BadgeRevoked};
pub use storage::DataKey;
pub use types::{Badge, BadgeStatus};
