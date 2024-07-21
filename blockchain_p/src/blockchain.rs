use block::Block;
use transaction::Transaction;

mod block;
mod transaction;

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Self {
            chain: Vec::new(),
            current_transaction: Vec::new(),
        };
        // ADD GENESIS BLOCK
        bc.new_block(100, Some(String::from("1")));
        bc
    }

    /// returns clone of Vec<Block>
    pub fn get_chain(&self) -> Vec<Block> {
        self.chain.clone()
    }

    /// Create hash for blocks - Sha256
    pub fn hash(block: &Block) -> String {
        sha256::digest(format!("{:?}", block))
    }

    /// proof: The prrof given by the Proof of Work Algo
    /// previous_hash: Hash of previous block
    pub fn new_block(&mut self, proof: u32, previous_hash: Option<String>) -> Block {
        let new_index = self.chain.len();
        let last_block = self.last_block();
        self.chain.push(Block::new(
            new_index,
            self.current_transaction.clone(),
            proof,
            previous_hash.unwrap_or(match last_block {
                Some(b) => Blockchain::hash(&b),
                None => "1".into(),
            }),
        ));
        self.current_transaction = Vec::new();

        self.last_block().unwrap()
    }

    // returns usize - index of the block that will be creted with this transaction
    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: usize) -> usize {
        self.current_transaction
            .push(Transaction::new(sender, receiver, amount));
        self.last_block().unwrap().index + 1
    }

    pub fn last_block(&self) -> Option<Block> {
        self.chain.last().cloned()
    }

    pub fn proof_of_work(&self, last_proof: u32) -> u32 {
        let mut proof = 0;
        while !Self::validate_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    /// 4 leading string is "8888"
    fn validate_proof(last_proof: u32, proof: u32) -> bool {
        let guess = sha256::digest(format!("{}{}", last_proof, proof));
        println!("{}", guess);
        guess[..4] == *"8888"
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
