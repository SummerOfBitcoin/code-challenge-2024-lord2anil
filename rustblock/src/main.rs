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
use mine_block::*;
use std::path::PathBuf;
// mod validation_scripts::{p2pkh};
use std::io::BufRead;
use mine_block::mine_block;
mod validation_scripts;


use utiles::{
    convert_to_4bytes, reverse_bytes, serialize_coinbase_transaction, txid_data,
    write_to_output_file,
};

fn serialize_block_header(block: &Block) -> String {
    format!(
        "{}{}{}{}{}{}",
        (block.version.clone()),
        (block.prev_block_hash.clone()),
        (block.merkle_root.clone()),
        (block.timestamp.clone()),
        (block.bits.clone()),
        (convert_to_4bytes(block.nonce))
    )
}

fn main() {
    let folder_path = "../mempool2";

    let mut transactions: Vec<Transaction> = Vec::new();

    let mut x = 0;

    for entry in std::fs::read_dir(folder_path).unwrap() {
        x = x + 1;
        let entry = entry.unwrap();
        let path = entry.path();
        let path_clone = path.clone(); // Clone the path variable

        let mut file = File::open(path_clone).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let transaction: Transaction = match serde_json::from_str::<Transaction>(&data) {
            Ok(result) => result,
            Err(_e) => {
                //    println!("{}",e);
                continue;
            }
        };

        transactions.push(transaction);
    }

    // let sample_file_name="../valid.txt";

    // // Read the file names from t.txt, line by line
    // let file = File::open(sample_file_name).unwrap();
    // let reader = std::io::BufReader::new(file);
    // let mut valid_file_names: Vec<PathBuf> = Vec::new();

    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let pp = format!("../mempool/{}", line);
    //     let path = PathBuf::from(pp); // Construct PathBuf directly
    //     valid_file_names.push(path);
    // }
    // valid_file_names.pop();
    // let mut p=true;
    // let xxxx =valid_file_names.len()-1;
    // for path in valid_file_names {
    //    if p==true {
    //        p=false;
    //        continue;
    //    }
    //   x=x+1;
    //     let mut file = File::open(path).unwrap();
    //     let mut data = String::new();
    //     file.read_to_string(&mut data).unwrap();

    //     let transaction: Transaction = match serde_json::from_str::<Transaction>(&data) {

    //         Ok(result) => result,
    //         Err(_e) => {

    //         //    println!("{}",e);
    //             continue;
    //         }

    //     };

    //     transactions.push(transaction);
    // }
    

    transactions = validate_transactions(&transactions).clone();

    println!("{:?} { }", transactions.len(),x);

    let coinbase_transaction: Transaction =
        construct_coinbase_transaction(6, 1, transactions.clone()); // Example block reward and transaction fees
    transactions.insert(0, coinbase_transaction);
    let block = assemble_block(transactions);

    let difficulty_target = "0000ffff00000000000000000000000000000000000000000000000000000000";

    // Mine the block
    let mined_block = mine_block(block, difficulty_target.to_string());

    // generate the block header
    let block_header = serialize_block_header(&mined_block);

    // Serialize the coinbase transaction
    let coinbase_tx = serialize_coinbase_transaction(&mined_block.transactions[0]);
    let txids = calculate_txid(&mined_block.transactions);

    // println!("{}", coinbase_tx);
    // Write block data to output.txt file
    write_to_output_file(block_header, &coinbase_tx, txids);
}

// Function to calculate Txids  of  all transactions
fn calculate_txid(transactions: &[Transaction]) -> Vec<String> {
    let mut txids = vec![];

    for t in transactions {
        let txid = txid_data(t.clone());
        txids.push(reverse_bytes(txid));
    }
    txids
}
