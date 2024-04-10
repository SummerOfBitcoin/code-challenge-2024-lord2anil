
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
        let  path = entry.path();
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
   
    transactions= validate_transactions(&transactions).clone();
    println!("{:?} {x}", transactions.len());


 //   // let coinbase_transaction: Transaction = construct_coinbase_transaction(1000000, 50000); // Example block reward and transaction fees
  //  let block = assemble_block(transactions);

    // Print the assembled block
    // println!("{:#?}", block);
//   let difficulty_target = "0000ffff00000000000000000000000000000000000000000000000000000000";

    // Mine the block
  //  let mined_block = mine_block(block, difficulty_target);

    // Print the mined block
    // println!("{:#?}", mined_block);
  //  let coinbase_txid = "123abc";
   // let transaction_txids = vec!["456def".to_string(), "789ghi".to_string()];

    // Write block data to output.txt file
   // write_to_output_file(&mined_block, coinbase_txid, &transaction_txids);
}

   

use std::io::Write;

// Function to write block data to output.txt file
fn write_to_output_file(block: &Block, coinbase_txid: &str, transaction_txids: &[String]) {
    let mut file = match File::create("../output.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("Error creating output.txt file");
            return;
        }
    };

    // Write block header to file
    writeln!(file, "Block Header: {}", block.header).expect("Error writing to file");

    // Write serialized coinbase transaction to file
    writeln!(file, "Serialized Coinbase Transaction: {}", coinbase_txid).expect("Error writing to file");

    // Write transaction IDs of mined transactions to file
    writeln!(file, "Transaction IDs:").expect("Error writing to file");
    for txid in transaction_txids {
        writeln!(file, "{}", txid).expect("Error writing to file");
    }
}




