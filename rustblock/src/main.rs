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
    calculate_txids, reverse_bytes, serialize_block_header, serialize_coinbase_transaction,
    write_to_output_file,
};
fn main() {
    let folder_path = "../mempool";

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
        let weight=calculate_weight(&transaction);
        transaction.weight=weight;

        transactions.push(transaction);
    }

    // sort the transactions by fees, max fees first
    // transactions.sort_by(|a, b| b.fees.cmp(&a.fees));
    transactions = validate_transactions(&transactions).clone();

    let mut needed_transactions: Vec<Transaction> = Vec::new();
    let mut extra_transactions: Vec<Transaction> = Vec::new();
    let mut cnt=0;
     for x in transactions.iter(){
        cnt+=1;
        if cnt<=3200{
            needed_transactions.push(x.clone());
        }else{
            extra_transactions.push(x.clone());
        }   }
    for i in 0..needed_transactions.len(){
        let wt=needed_transactions[i].weight;
        // find lower bound of that  weight in extra transactions , and if it has hight fees than replace it with that
        let mut max_fees=0;
        let mut max_index=0;
        for j in 0..extra_transactions.len(){
            if extra_transactions[j].weight<wt{
                if extra_transactions[j].fees>max_fees{
                    max_fees=extra_transactions[j].fees;
                    max_index=j;
                }
            }
        }
        if max_fees>0{
            let xx= needed_transactions[i].clone();
            needed_transactions[i]=extra_transactions[max_index].clone();
            extra_transactions[max_index]=xx.clone();
        }
    }
    
       



    println!("{:?} { }", transactions.len(), x);
    let mut transactions: Vec<Transaction> = needed_transactions.clone();

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
     let txids:Vec<String> = txids.iter().map(|txid| reverse_bytes(txid.clone())).collect();

    // Write block data to output.txt file
    write_to_output_file(block_header, &coinbase_tx, txids);
}


// calculate the fees
fn calculate_fees(transaction: &Transaction) -> u64 {
    let mut total_input: u64 = 0;
    let mut total_output: u64 = 0;

    for input in &transaction.vin {
        total_input += input.prevout.value;
    }

    for output in &transaction.vout {
        total_output += output.value;
    }

    total_input - total_output
}


fn calculate_weight(transaction: &Transaction)->u64{
    let non_seginput_weight=272;
    let seg_input_weight=720;
    let output_weight=124;
    let mut total_weight:u64=0;
    for input in &transaction.vin{
        if input.witness.is_empty(){
            total_weight+=non_seginput_weight;
        }else{
            total_weight+=seg_input_weight;
        }
    }
    total_weight+=output_weight*transaction.vout.len() as u64;
    total_weight
    
}

