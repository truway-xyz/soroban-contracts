use soroban_sdk::{contractevent, Address, Symbol};

/// Published on every `allocate`. Topics: `("fund_allocated", project_id)`.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundAllocated {
    #[topic]
    pub project_id: Symbol,
    pub organization: Address,
    pub amount_xlm: i128,
    pub allocated_at: u64,
}
