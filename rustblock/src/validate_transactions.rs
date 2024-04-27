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
    while total_weight > 3970000 {
        // remove the first transaction
        total_weight = total_weight - valid_transactions[0].weight as u64;
        valid_transactions.remove(0);
    }
    println!("{}", total_weight);

    valid_transactions
}

fn is_valid_transaction(t: &Transaction) -> bool {
    if t.vin.len() == 0 || t.vout.len() == 0 {
        return false;
    }
    let mut p2wpkh = 0;
    let mut p2pkh = 0;
    for i in 0..t.vin.len() {
        // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe

        if t.vin[i].prevout.scriptpubkey_type != "v0_p2wpkh".to_string() {
            p2wpkh = p2wpkh + 1;
        }
        if t.vin[i].prevout.scriptpubkey_type != "p2pkh".to_string() {
            p2pkh = p2pkh + 1;
        }
    }

    // println!(" this is pwpkh transaction ");
    if p2wpkh == 0 {
        for i in 0..t.vin.len() {
            // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe

            if t.vin[i].prevout.scriptpubkey_type == "v0_p2wpkh".to_string() {
                if !p2wpkh_validate(t, i) {
                    return false;
                }
            }
        }
    } else if p2pkh == 0 {
        for i in 0..t.vin.len() {
            // println!("{}",t.vin[i].prevout.scriptpubkey_type);brfe

            if t.vin[i].prevout.scriptpubkey_type == "p2pkh".to_string() {
                if !p2pkh_validate(t, i) {
                    return false;
                }
            }
        }
    } else {
        return false;
    }

    // println!("{}",t.vin[0].txid);
    true
}
