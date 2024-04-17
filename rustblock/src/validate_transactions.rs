use core::panic;

use ripemd::Ripemd160;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};

use crate::Transaction;

use super::validations_utiles::*;

use super::utiles::{convert_to_4bytes, convert_to_8bytes, int_to_varint};


pub fn validate_transactions(transactions: &[Transaction]) -> Vec<Transaction> {
    let mut valid_transactions: Vec<Transaction> = Vec::new();
    for transaction in transactions {
        if is_valid_transaction(transaction) {
            valid_transactions.push(transaction.clone());
        }
    }

    valid_transactions
}








fn p2pkh_validate(t: &Transaction, idx: usize) -> bool {
    let scriptsig_asm1 = t.vin[idx].scriptsig_asm.clone();
    let binding = scriptsig_asm1.split(" ").collect::<Vec<&str>>();
    let pub_key = binding.last().unwrap();
    let pub_key = hex::decode(pub_key).unwrap();
    let pub_key_hash256 = Sha256::digest(pub_key);
    let pub_key_ripemd160 = Ripemd160::digest(&pub_key_hash256);
    let pub_key_ripemd160_hex = hex::encode(pub_key_ripemd160);
    let pub_key_hash = t.vin[idx]
        .prevout
        .scriptpubkey_asm
        .split(" ")
        .collect::<Vec<&str>>()[3];
    // println!("{}   {}", pub_key_ripemd160_hex, pub_key_hash);
    if pub_key_ripemd160_hex != pub_key_hash {
       
        return false;
    }
    if !p2pkh_verify_signature(t.clone(), idx) {
       
        return false;
    }

    
    true
}



fn is_valid_transaction(t: &Transaction) -> bool {
    if t.vin.len() == 0 || t.vout.len() == 0 {
        return false;
    }
    let mut cnt=0;
    for i in 0..t.vin.len() {
        // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe
       
        if t.vin[i].prevout.scriptpubkey_type != "p2pkh".to_string() {
            
           cnt=cnt+1;
        }
    }
    if cnt>0{
        return false;
    }

    for i in 0..t.vin.len() {
        // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe
       
        if t.vin[i].prevout.scriptpubkey_type == "p2pkh".to_string() {
            
            if !p2pkh_validate(t, i) {
                // println!("invalid signature");
                
                return false;
            }
        }
    }
    // println!("{}",t.vin[0].txid);
    true
}
