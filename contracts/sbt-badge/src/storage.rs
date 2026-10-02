use soroban_sdk::{contracttype, Address, Env, Vec};
use truway_common::ttl;

use crate::types::Badge;

/// Storage keys. Variant names are part of the on-ledger encoding: never
/// rename or remove one used by a deployed instance without a migration.
#[contracttype]
pub enum DataKey {
    Admin,
    NextTokenId,
    BadgeById(u64),
    OrgBadges(Address),
}

pub(crate) fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub(crate) fn require_admin(env: &Env) {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("not_initialized"));
    admin.require_auth();
}

pub(crate) fn next_token_id(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::NextTokenId).unwrap()
}

pub(crate) fn set_next_token_id(env: &Env, token_id: u64) {
    env.storage()
        .instance()
        .set(&DataKey::NextTokenId, &token_id);
}

pub(crate) fn get_badge(env: &Env, token_id: u64) -> Option<Badge> {
    env.storage()
        .persistent()
        .get(&DataKey::BadgeById(token_id))
}

pub(crate) fn set_badge(env: &Env, badge: &Badge) {
    let key = DataKey::BadgeById(badge.token_id);
    env.storage().persistent().set(&key, badge);
    ttl::extend_persistent(env, &key);
}

/// Index of an organization's badges: organization → token ids.
pub(crate) fn org_badge_ids(env: &Env, organization: Address) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::OrgBadges(organization))
        .unwrap_or_else(|| Vec::new(env))
}

pub(crate) fn set_org_badge_ids(env: &Env, organization: Address, ids: &Vec<u64>) {
    let key = DataKey::OrgBadges(organization);
    env.storage().persistent().set(&key, ids);
    ttl::extend_persistent(env, &key);
}
