// Function to mine a block
extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::transacton_struct::*;
use crate::assemble_block::*;



pub fn mine_block(mut block: Block, difficulty_target: &str) -> Block {
    let mut hasher = Sha256::new();

    loop {
        // Concatenate block header and nonce
        let data = format!("{}{}", block.header, block.nonce);

        // Calculate hash of concatenated data
        hasher.input_str(&data);
        let hash = hasher.result_str();
        hasher.reset();

        // Check if the hash meets the difficulty target
        if hash.starts_with(difficulty_target) {
            block.header = hash;
            break; // Block mined successfully
        }

        // Increment nonce for next iteration
        block.nonce += 1;
    }

    block
}