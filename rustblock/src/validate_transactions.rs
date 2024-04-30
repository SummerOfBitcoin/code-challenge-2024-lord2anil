use crate::Transaction;

use super::validation_scripts::p2pkh::p2pkh_validate;
use super::validation_scripts::p2wpkh::p2wpkh_validate;

pub fn validate_transactions(transactions: &[Transaction]) -> Vec<Transaction> {
    let mut valid_transactions: Vec<Transaction> = Vec::new();
    let mut total_weight: u64 = 0;
    for transaction in transactions {
        if is_valid_transaction(transaction) {
            total_weight = total_weight + transaction.weight as u64;
            valid_transactions.push(transaction.clone());
            if total_weight > 4000000 {
                break;
            }
        }
    }
    // remove the first transaction if the total weight is greater than 4,000,000
    // i am doing "3970000"  this because the last transaction is always the coinbase transaction and i know the weight of the coinbase transaction at this point
    while total_weight > 3970000 {
        // remove the first transaction
        total_weight = total_weight - valid_transactions[0].weight as u64;
        valid_transactions.remove(0);
    }

    valid_transactions
}

fn is_valid_transaction(t: &Transaction) -> bool {
    if t.vin.len() == 0 || t.vout.len() == 0 {
        return false;
    }
    let mut p2wpkh = 0;
    let mut p2pkh = 0;
    // i am implemented the script validation for p2wpkh and p2pkh only, 
    for i in 0..t.vin.len() {

        if t.vin[i].prevout.scriptpubkey_type != "v0_p2wpkh".to_string() {
            p2wpkh = p2wpkh + 1;
        }
        if t.vin[i].prevout.scriptpubkey_type != "p2pkh".to_string() {
            p2pkh = p2pkh + 1;
        }
    }
// if the transaction is not p2wpkh or p2pkh return false
// if it is p2wpkh or p2pkh validate the transaction
    if p2wpkh == 0 {
        for i in 0..t.vin.len() {

            if t.vin[i].prevout.scriptpubkey_type == "v0_p2wpkh".to_string() {
                if !p2wpkh_validate(t, i) {
                    return false;
                }
            }
        }
    } else if p2pkh == 0 {
        for i in 0..t.vin.len() {

            if t.vin[i].prevout.scriptpubkey_type == "p2pkh".to_string() {
                if !p2pkh_validate(t, i) {
                    return false;
                }
            }
        }
    } else {
        return false;
    }
    true
}
