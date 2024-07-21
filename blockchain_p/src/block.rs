use std::time::{SystemTime, UNIX_EPOCH};

use crate::transaction::Transaction;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transaction: Transaction,
    pub proof: u32,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: u32, tx: Transaction, proof: u32, prev_hash: String) -> Self {
        Self {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transaction: tx,
            proof,
            previous_hash: prev_hash,
        }
    }
}
