use std::io::Read;

use serde_json;
use std::fs::File;
mod assemble_block;
mod coinbase_transaction;
mod validate_transactions;
use assemble_block::*;
use coinbase_transaction::*;
use validate_transactions::validate_transactions;
mod transacton_struct;
mod utiles;
use transacton_struct::Transaction;
mod mine_block;
use mine_block::mine_block;
mod validation_scripts;

use utiles::{
    calculate_fees, calculate_txids, calculate_weight, reverse_bytes, serialize_block_header,
    serialize_coinbase_transaction, write_to_output_file,
};
fn main() {
    let folder_path = "../mempool";

    let mut transactions: Vec<Transaction> = Vec::new();

    let mut x = 0;
    for entry in std::fs::read_dir(folder_path).unwrap() {
        x = x + 1;
        let entry = entry.unwrap();
        let path = entry.path();
        let path_clone = path.clone();

        let mut file = File::open(path_clone).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        // serde_json to parse the json data
        let mut transaction: Transaction = match serde_json::from_str::<Transaction>(&data) {
            Ok(result) => result,
            Err(_e) => {
                continue;
            }
        };
        // calculate the fees
        let fees = calculate_fees(&transaction);
        transaction.fees = fees;
        // calculate the weight
        let weight = calculate_weight(&transaction);
        transaction.weight = weight;
        transactions.push(transaction);
    }

    // sort the transactions by fees, max fees first
    transactions.sort_by(|a, b| b.weight.cmp(&a.weight));
    transactions = validate_transactions(&transactions).clone();

    println!("{:?} { }", transactions.len(), x);
    // filter the transactions to get high score
    // let mut transactions: Vec<Transaction> = filter_transactions(&transactions);

    // Construct the coinbase transaction
    let coinbase_transaction: Transaction =
        construct_coinbase_transaction(200000, 100000, transactions.clone());
    transactions.insert(0, coinbase_transaction);
    let block = assemble_block(transactions);

    let difficulty_target = "0000ffff00000000000000000000000000000000000000000000000000000000";

    // Mine the block
    let mined_block = mine_block(block, difficulty_target.to_string());

    // generate the block header
    let block_header = serialize_block_header(&mined_block);

    // Serialize the coinbase transaction
    let coinbase_tx = serialize_coinbase_transaction(&mined_block.transactions[0]);
    let txids = calculate_txids(&mined_block.transactions);
    // convert txids in reverse order to bytes
    let txids: Vec<String> = txids
        .iter()
        .map(|txid| reverse_bytes(txid.clone()))
        .collect();

    // Write block data to output.txt file
    write_to_output_file(block_header, &coinbase_tx, txids);
}
