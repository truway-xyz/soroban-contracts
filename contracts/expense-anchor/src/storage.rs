use soroban_sdk::{contracttype, Address, Env, Symbol};
use truway_common::ttl;

use crate::types::AnchoredExpense;

/// Storage keys. Variant names are part of the on-ledger encoding: never
/// rename or remove one used by a deployed instance without a migration.
#[contracttype]
pub enum DataKey {
    Expense(Symbol),
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
// allowed to write. Without the admin check any address could anchor forged
// expenses under any project_id.
pub(crate) fn require_caller_is_admin(env: &Env, caller: &Address) {
    if caller != &admin(env) {
        panic!("not_admin");
    }
    caller.require_auth();
}

pub(crate) fn get_expense(env: &Env, expense_id: Symbol) -> Option<AnchoredExpense> {
    env.storage()
        .persistent()
        .get(&DataKey::Expense(expense_id))
}

pub(crate) fn set_expense(env: &Env, expense_id: Symbol, expense: &AnchoredExpense) {
    let key = DataKey::Expense(expense_id);
    env.storage().persistent().set(&key, expense);
    ttl::extend_persistent(env, &key);
}

pub(crate) fn extend_expense(env: &Env, expense_id: Symbol) {
    ttl::extend_persistent(env, &DataKey::Expense(expense_id));
}
