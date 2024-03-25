



use crate::Transaction;

#[derive(Debug)]
pub struct Block {
   pub header: String, // Placeholder for block header
   pub transactions: Vec<Transaction>,
   pub nonce: u64,
  pub total_fee: u64,
}

// // Function to assemble a block
// pub fn assemble_block(transactions: Vec<Transaction>, coinbase_transaction: Transaction) -> Block {
//     // Add the coinbase transaction to the list of transactions
//     let mut all_transactions = vec![coinbase_transaction];
//     all_transactions.extend(transactions);

//     // Calculate the merkle root of the transactions
//     let merkle_root = calculate_merkle_root(&all_transactions);

//     // Create a block header (placeholder for simplicity)
//     let block_header = format!("Merkle Root: {}", merkle_root);

//     // Assemble the block
//     let block = Block {
//         header: block_header,
//         transactions: all_transactions,
//         nonce: 0,
//     };

//     block
// }
// Adjust the assemble_block function to exclude coinbase transaction
pub fn assemble_block(transactions: Vec<Transaction>) -> Block {
    // Calculate the merkle root of the transactions
    let merkle_root = calculate_merkle_root(&transactions);

    // Create a block header (placeholder for simplicity)
    let block_header = format!("Merkle Root: {}", merkle_root);

    // Assemble the block
    let block = Block {
        header: block_header,
        transactions: transactions,
        nonce: 0,
        total_fee: 0,
    };

    block
}


use sha2::{Digest, Sha256};

// Function to calculate Merkle root of transactions
fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    // If there are no transactions, return a default Merkle root value
    if transactions.is_empty() {
        return "0000000000000000000000000000000000000000000000000000000000000000".to_string();
    }

    // Helper function to compute the hash of two concatenated hashes
    fn hash_concat(hash1: &[u8], hash2: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(hash1);
        hasher.update(hash2);
        hasher.finalize().to_vec()
    }

    // Helper function to compute Merkle root recursively
    fn compute_merkle_root(hashes: &[Vec<u8>]) -> Vec<u8> {
        // Base case: if there's only one hash, return it
        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        // Create a new vector to store hashes of the next level
        let mut next_level_hashes = Vec::new();

        // Iterate through pairs of hashes and compute their combined hash
        for chunk in hashes.chunks(2) {
            // If there's only one hash in the chunk, duplicate it
            let hash1 = &chunk[0];
            let hash2 = if chunk.len() == 1 {
                chunk[0].clone()
            } else {
                chunk[1].clone()
            };

            // Compute the combined hash of the two hashes
            let combined_hash = hash_concat(hash1, hash2.as_slice());
            // Hash the combined hash to get the next-level hash
            let next_level_hash = Sha256::digest(&combined_hash);

            // Add the next-level hash to the list of next level hashes
            next_level_hashes.push(next_level_hash.to_vec());
        }

        // Recursively compute the Merkle root of the next level
        compute_merkle_root(&next_level_hashes)
    }

    // Extract transaction hashes
    let transaction_hashes: Vec<Vec<u8>> = transactions
        .iter()
        .map(|transaction| {
            // For simplicity, concatenate all fields of the transaction and hash them
            let data = format!("{:?}", transaction);
            Sha256::digest(data.as_bytes()).to_vec()
        })
        .collect();

    // Compute the Merkle root recursively
    let merkle_root = compute_merkle_root(&transaction_hashes);

    // Convert the Merkle root hash to a hexadecimal string
    hex::encode(merkle_root)
}

