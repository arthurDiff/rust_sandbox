use std::time::{SystemTime, UNIX_EPOCH};

use super::transaction::Transaction;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: u32,
    pub previous_hash: String,
}

impl Block {
    pub fn new(
        index: usize,
        transactions: Vec<Transaction>,
        proof: u32,
        previous_hash: String,
    ) -> Self {
        Self {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            proof,
            previous_hash,
        }
    }
}
