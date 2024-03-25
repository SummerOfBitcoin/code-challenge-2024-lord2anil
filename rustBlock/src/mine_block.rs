use sha2::{Digest, Sha256};
use hex;
use crate::transacton_struct::*;
use crate::assemble_block::*;

// Function to calculate the total fee collected from transactions
fn calculate_total_fee(transactions: &[Transaction]) -> u64 {
    transactions.iter().map(|tx| calculate_transaction_fee(tx)).sum()
}

// Function to calculate the fee of a transaction
fn calculate_transaction_fee(transaction: &Transaction) -> u64 {
    let input_value: u64 = transaction.vin.iter().map(|input| input.prevout.value).sum();
    let output_value: u64 = transaction.vout.iter().map(|output| output.value).sum();
    input_value - output_value
}

pub fn mine_block(mut block: Block, difficulty_target: &str) -> Block {
    let mut hasher = Sha256::new();
    let mut x=1000;
    loop {
        x=x-1;
        // Concatenate block header and nonce
        let data = format!("{}{}", block.header, block.nonce);
        // Convert data to bytes
        let data_bytes = data.as_bytes();

        // Calculate hash of concatenated data
        let mut hasher = Sha256::new(); // Create a new instance of the hasher
        hasher.update(data_bytes);
        let hash = hasher.finalize();

        // Check if the hash meets the difficulty target
        if hash.starts_with(difficulty_target.as_bytes()) || x==0{
            // Convert hash to hexadecimal string
            block.header = hex::encode(hash);
            break; // Block mined successfully
        }

        // Increment nonce for next iteration
        block.nonce += 1;
    }

    // Calculate the total fee
    let transactions = &block.transactions;
    block.total_fee = calculate_total_fee(transactions);

    block
}
