use crate::{reverse_bytes, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

use super::utiles::{calculate_txids, convert_to_4bytes, merkle_root};

#[derive(Debug, Clone)]
pub struct Block {
    pub version: String,
    pub prev_block_hash: String,
    pub merkle_root: String,
    pub timestamp: String,
    pub bits: String,
    pub nonce: u32,
    pub transactions: Vec<Transaction>,
}

pub fn assemble_block(transactions: Vec<Transaction>) -> Block {
    // Calculate the merkle root of the transactions
    let txids = calculate_txids(&transactions);
    let merkle_root = merkle_root(txids.clone());

    // assemble the block
    let block = Block {
        version: "04000000".to_string(),
        prev_block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
            .to_string(),
        merkle_root: merkle_root,
        // unix timestamp
        timestamp: convert_to_4bytes(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        ),
        // this is the bits representation of the target string "0000ffff00000000000000000000000000000000000000000000000000000000"
        bits: reverse_bytes("1f00ffff".to_string()),
        nonce: 0,
        transactions: transactions,
    };

    block
}
