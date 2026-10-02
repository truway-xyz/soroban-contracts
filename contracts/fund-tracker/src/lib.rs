//! # fund-tracker
//!
//! Records the budget allocated to each project as an attested, timestamped
//! entry. See the crate README for the interface, storage layout and events.
#![no_std]

mod contract;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use contract::{FundTracker, FundTrackerClient};
pub use events::FundAllocated;
pub use storage::DataKey;
pub use types::FundAllocation;
