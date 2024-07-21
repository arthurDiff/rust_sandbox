use crate::transaction::Transaction;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transaction: Transaction,
    pub proof: u32,
    pub previous_hash: String,
}
