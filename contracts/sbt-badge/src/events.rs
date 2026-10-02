use soroban_sdk::{contractevent, Address, Symbol};

/// Topics: `("badge_minted", badge_type)`.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadgeMinted {
    #[topic]
    pub badge_type: Symbol,
    pub token_id: u64,
    pub organization: Address,
}

/// Topics: `("badge_revoked",)`.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadgeRevoked {
    pub token_id: u64,
    pub organization: Address,
}
