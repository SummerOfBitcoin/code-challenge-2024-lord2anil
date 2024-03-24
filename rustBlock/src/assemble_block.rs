



use crate::Transaction;

#[derive(Debug)]
pub struct Block {
    header: String, // Placeholder for block header
    transactions: Vec<Transaction>,
    nonce: u64,
}

// Function to assemble a block
pub fn assemble_block(transactions: Vec<Transaction>, coinbase_transaction: Transaction) -> Block {
    // Add the coinbase transaction to the list of transactions
    let mut all_transactions = vec![coinbase_transaction];
    all_transactions.extend(transactions);

    // Calculate the merkle root of the transactions
    let merkle_root = calculate_merkle_root(&all_transactions);

    // Create a block header (placeholder for simplicity)
    let block_header = format!("Merkle Root: {}", merkle_root);

    // Assemble the block
    let block = Block {
        header: block_header,
        transactions: all_transactions,
        nonce: 0,
    };

    block
}

// Placeholder function to calculate merkle root (replace with actual implementation)
fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    // Placeholder logic to calculate merkle root
    "0000000000000000000000000000000000000000000000000000000000000000".to_string()
}

