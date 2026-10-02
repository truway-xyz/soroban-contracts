use soroban_sdk::{contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum BadgeStatus {
    Active,
    Revoked,
}

#[contracttype]
#[derive(Clone)]
pub struct Badge {
    pub token_id: u64,
    pub badge_type: Symbol,
    pub organization: Address,
    pub status: BadgeStatus,
    pub issued_at: u64,
    pub revoked_at: u64, // 0 when not revoked
}

/// Badge types are compiled into the contract: adding one requires an upgrade.
pub(crate) fn is_valid_badge_type(env: &Env, badge_type: &Symbol) -> bool {
    let valid = [
        Symbol::new(env, "kyb_verified"),
        Symbol::new(env, "transparency_bronze"),
        Symbol::new(env, "transparency_silver"),
        Symbol::new(env, "transparency_gold"),
    ];
    valid.iter().any(|t| t == badge_type)
}
