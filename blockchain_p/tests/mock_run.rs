use blockchain_p::blockchain::Blockchain;

#[test]
fn mine_blockchain() {
    let mut bc = Blockchain::new();

    // Mine new block
    let (sender, receiver, amount) = ("me".to_string(), "you".to_string(), 1_usize);

    let last_block = bc
        .last_block()
        .expect("expected to get a block but did not");
    let last_proof = last_block.proof;
    assert_eq!(
        last_proof, 100,
        "expected last_block to be genesis block but got = {:?}",
        last_block
    );
    let proof = bc.proof_of_work(last_proof);
    let future_block_transaction_index = bc.new_transaction(sender, receiver, amount);
    let new_block = bc.new_block(proof, Some(Blockchain::hash(&last_block)));
    assert_eq!(
        new_block.index, future_block_transaction_index,
        "expected newly created block to be index 1 but got = {:?}",
        new_block
    );
}
