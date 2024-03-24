
use serde::{Deserialize};
use std::fs::File;
use std::io::Read;
use serde_json;
mod validate_transactions;
use validate_transactions::validate_transactions;


mod transacton_struct;

use transacton_struct::Transaction;

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

   
}
