//! # expense-anchor
//!
//! Anchors approved expenses and donor reports as immutable records that carry
//! the SHA-256 hash of their receipt. See the crate README for the interface,
//! storage layout and events.
#![no_std]

mod contract;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use contract::{ExpenseAnchor, ExpenseAnchorClient};
pub use events::ExpenseAnchored;
pub use storage::DataKey;
pub use types::AnchoredExpense;
