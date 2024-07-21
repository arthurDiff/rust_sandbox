use crate::{block::Block, transaction::Transaction};

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        // ADD GENESIS BLOCK
        let mut bc = Self {
            chain: Vec::new(),
            current_transaction: Vec::new(),
        };
        bc.new_block(100, Some(String::from("1")));
        bc
    }
    pub fn hash(block: Block) {
        todo!()
    }

    /// proof: The prrof given by the Proof of Work Algo
    /// previous_hash: Hash of previous block
    pub fn new_block(&mut self, proof: u32, previous_hash: Option<String>) {
        todo!()
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: usize) -> u32 {
        self.current_transaction
            .push(Transaction::new(sender, receiver, amount));
        //Check why return last block index
        match self.last_block() {
            Some(b) => b.index + 1,
            None => 1,
        }
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
