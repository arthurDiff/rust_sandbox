#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// sender address
    pub sender: String,
    /// receiver address
    pub receiver: String,
    pub amount: usize,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: usize) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}
