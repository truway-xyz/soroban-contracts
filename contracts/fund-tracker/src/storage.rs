use soroban_sdk::{contracttype, Address, Env, Symbol};
use truway_common::ttl;

use crate::types::FundAllocation;

/// Storage keys. Variant names are part of the on-ledger encoding: never
/// rename or remove one used by a deployed instance without a migration.
#[contracttype]
pub enum DataKey {
    Allocation(Symbol),
    Admin,
}

pub(crate) fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub(crate) fn admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("not_initialized"))
}

// `require_auth` alone only proves that *someone* signed, not that they are
// allowed to write. Without the admin check any address could overwrite the
// allocation of any project.
pub(crate) fn require_caller_is_admin(env: &Env, caller: &Address) {
    if caller != &admin(env) {
        panic!("not_admin");
    }
    caller.require_auth();
}

pub(crate) fn get_allocation(env: &Env, project_id: Symbol) -> Option<FundAllocation> {
    env.storage()
        .persistent()
        .get(&DataKey::Allocation(project_id))
}

pub(crate) fn set_allocation(env: &Env, project_id: Symbol, allocation: &FundAllocation) {
    let key = DataKey::Allocation(project_id);
    env.storage().persistent().set(&key, allocation);
    ttl::extend_persistent(env, &key);
}
