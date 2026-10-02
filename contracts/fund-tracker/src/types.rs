use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct FundAllocation {
    pub project_id: Symbol,
    pub organization: Address,
    pub amount_xlm: i128,
    pub allocated_at: u64,
}
