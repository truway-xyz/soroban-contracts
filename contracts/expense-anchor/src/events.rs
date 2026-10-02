use soroban_sdk::{contractevent, Address, Bytes, Symbol};

/// Published on every new anchor. Topics: `("expense_anchored", project_id)`.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpenseAnchored {
    #[topic]
    pub project_id: Symbol,
    pub expense_id: Symbol,
    pub submitted_by: Address,
    pub amount_xlm: i128,
    pub receipt_hash: Bytes,
    pub anchored_at: u64,
}
