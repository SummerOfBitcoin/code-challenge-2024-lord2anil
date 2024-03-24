
use serde::{Deserialize};
use std::fs::File;
use std::io::Read;
use serde_json;
mod validate_transactions;
mod assemble_block;
mod coinbase_transaction;
use validate_transactions::validate_transactions;
use assemble_block::*;
use coinbase_transaction::*;
mod transacton_struct;

use transacton_struct::Transaction;
mod mine_block;
use mine_block::*;

fn main() {
    let folder_path= "../mempool";

    let mut transactions: Vec<Transaction> = Vec::new();
    let mut x=0;
    for entry in std::fs::read_dir(folder_path).unwrap() {
        x=x+1   ;
        let entry = entry.unwrap();
        let path = entry.path();
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let transaction: Transaction = match serde_json::from_str::<Transaction>(&data) {
            Ok(result) => result,
            Err(e) => {
                continue;
            }
        };
        
        transactions.push(transaction);
    }
    transactions= validate_transactions(&transactions).clone();
    println!("{:?} {x}", transactions.len());
    let coinbase_transaction: Transaction = construct_coinbase_transaction(1000000, 50000); // Example block reward and transaction fees
    let block = assemble_block(transactions, coinbase_transaction);

    // Print the assembled block
    // println!("{:#?}", block);
    let difficulty_target = "0000ffff00000000000000000000000000000000000000000000000000000000";

    // Mine the block
    let mined_block = mine_block(block, difficulty_target);

    // Print the mined block
    println!("{:#?}", mined_block);
}

   

