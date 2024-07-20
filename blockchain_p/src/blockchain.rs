use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<String>,
    current_transaction: Vec<String>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            current_transaction: Vec::new(),
        }
    }

    pub fn hash(block: Block) {
        todo!()
    }

    pub fn last_block(&self) {
        todo!()
    }
}
