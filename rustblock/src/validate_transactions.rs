use core::panic;

use ripemd::Ripemd160;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};

use crate::Transaction;





use super::validation_scripts::p2pkh::p2pkh_validate;
use super::validation_scripts::p2wpkh::p2wpkh_validate;


pub fn validate_transactions(transactions: &[Transaction]) -> Vec<Transaction> {
    let mut valid_transactions: Vec<Transaction> = Vec::new();
    for transaction in transactions {
        if is_valid_transaction(transaction) {
            valid_transactions.push(transaction.clone());
        }
        if valid_transactions.len() == 1500 {
            break;
        }
    }

    valid_transactions
}












fn is_valid_transaction(t: &Transaction) -> bool {
    if t.vin.len() == 0 || t.vout.len() == 0 {
        return false;
    }
    let mut cnt=0;
    for i in 0..t.vin.len() {
        // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe
       
        if t.vin[i].prevout.scriptpubkey_type != "v0_p2wpkh".to_string() {
            
           cnt=cnt+1;
        }
    }
    
    if cnt>0{
        return false;
    }
    // println!(" this is pwpkh transaction ");

    for i in 0..t.vin.len() {
        // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe
       
        if t.vin[i].prevout.scriptpubkey_type == "v0_p2wpkh".to_string() {
            
            if !p2wpkh_validate(t, i) {
           
                
                return false;
            }
        }
    }
    // println!("{}",t.vin[0].txid);
    true
}
