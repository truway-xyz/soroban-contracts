use soroban_sdk::{contracttype, Address, Bytes, Symbol};

/// Length of a SHA-256 digest, in bytes.
pub(crate) const RECEIPT_HASH_LEN: u32 = 32;

#[contracttype]
#[derive(Clone)]
pub struct AnchoredExpense {
    pub expense_id: Symbol,
    pub project_id: Symbol,
    pub submitted_by: Address,
    pub amount_xlm: i128,
    pub receipt_hash: Bytes, // SHA-256 of the receipt file
    pub anchored_at: u64,
}
