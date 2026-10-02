//! Storage TTL policy shared by all contracts.
//!
//! Every write extends the entry it touches, and the contract instance, so
//! that records stay live without a separate maintenance job. An entry is
//! extended once its remaining TTL drops below [`BUMP_THRESHOLD`], up to
//! [`BUMP_TO`]. See `docs/storage-and-ttl.md`.

use soroban_sdk::{Env, IntoVal, Val};

/// Ledgers closed in one day, at roughly five seconds per ledger.
pub const DAY_IN_LEDGERS: u32 = 17_280;

/// Remaining TTL below which an entry is extended (30 days).
pub const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;

/// TTL an entry is extended to (120 days).
pub const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

/// Extends the TTL of the contract instance and its code.
pub fn extend_instance(env: &Env) {
    env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
}

/// Extends the TTL of an existing persistent entry.
pub fn extend_persistent<K>(env: &Env, key: &K)
where
    K: IntoVal<Env, Val>,
{
    env.storage()
        .persistent()
        .extend_ttl(key, BUMP_THRESHOLD, BUMP_TO);
}
