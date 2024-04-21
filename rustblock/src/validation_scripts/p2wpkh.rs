use ripemd::Ripemd160;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};

use crate::Transaction;

use crate::utiles::{convert_to_4bytes, convert_to_8bytes, int_to_varint, reverse_bytes};

pub fn p2wpkh_validate(t: &Transaction, idx: usize) -> bool {
    let witness = t.vin[idx].witness.clone();

    let pub_key = witness.last().unwrap();
    let pub_key = hex::decode(pub_key).unwrap();
    let pub_key_hash256 = Sha256::digest(pub_key);
    let pub_key_ripemd160 = Ripemd160::digest(&pub_key_hash256);
    let pub_key_ripemd160_hex = hex::encode(pub_key_ripemd160);
    let pub_key_hash = t.vin[idx]
        .prevout
        .scriptpubkey_asm
        .split(" ")
        .collect::<Vec<&str>>()[2];
    // println!("{}   {}", pub_key_ripemd160_hex, pub_key_hash);
    if pub_key_ripemd160_hex != pub_key_hash {
        return false;
    }

    if !p2wpkh_verify_signature(t.clone(), idx) {
        //    println!("{}","signature verification failed");

        return false;
    }

    true
}

fn p2wpkh_verify_signature(t: Transaction, idx: usize) -> bool {
    // this is the seriliztions of the segwit transaction
    let mut transaction_data = String::new();
    // 4 bits version, in little endian
    transaction_data.push_str(&convert_to_4bytes(t.version));
    // hashPrevouts
    let mut hash_prevouts = String::new();
    for i in 0..t.vin.len() {
        hash_prevouts.push_str(&reverse_bytes(t.vin[i].txid.clone()));
        hash_prevouts.push_str(&convert_to_4bytes(t.vin[i].vout));
    }
    let hash_prevouts = Sha256::digest(Sha256::digest(hex::decode(hash_prevouts).unwrap()));
    transaction_data.push_str(&hex::encode(hash_prevouts));
    // hashSequence
    let mut hash_sequence = String::new();
    for i in 0..t.vin.len() {
        hash_sequence.push_str(&convert_to_4bytes(t.vin[i].sequence));
    }
    let hash_sequence = Sha256::digest(Sha256::digest(hex::decode(hash_sequence).unwrap()));
    transaction_data.push_str(&hex::encode(hash_sequence));
    // outpoint
    transaction_data.push_str(&reverse_bytes(t.vin[idx].txid.clone()));
    transaction_data.push_str(&convert_to_4bytes(t.vin[idx].vout));
    // scriptcode
    // And then the scriptCode, which, in P2WPKH’s case, is 1976a914 <pubkey hash> 88ac
    let pub_key_hash = t.vin[idx]
        .prevout
        .scriptpubkey_asm
        .split(" ")
        .collect::<Vec<&str>>()[2];
    let script_code = format!("1976a914{}88ac", pub_key_hash);
    // let script_code_len = script_code.len() / 2;
    // transaction_data.push_str(int_to_varint(script_code_len as u64).as_str());
    transaction_data.push_str(&script_code);
    // value
    transaction_data.push_str(&convert_to_8bytes(t.vin[idx].prevout.value as u64));
    // nSequence
    transaction_data.push_str(&convert_to_4bytes(t.vin[idx].sequence));
    // hashOutputs
    let mut hash_outputs = String::new();
    for i in 0..t.vout.len() {
        hash_outputs.push_str(&convert_to_8bytes(t.vout[i].value as u64));
        let scriptpubkey_len = t.vout[i].scriptpubkey.len() / 2;
        hash_outputs.push_str(int_to_varint(scriptpubkey_len as u64).as_str());
        hash_outputs.push_str(&t.vout[i].scriptpubkey);
    }
    let hash_outputs = Sha256::digest(Sha256::digest(hex::decode(hash_outputs).unwrap()));
    transaction_data.push_str(&hex::encode(hash_outputs));

    // nLocktime
    transaction_data.push_str(&convert_to_4bytes(t.locktime));
    // nHashType
    transaction_data.push_str("01000000");

    let tt = transaction_data.clone();
    // println!("{}",transaction_data);
    let transaction_hash = hex::decode(transaction_data).unwrap_or_else(|_e| {
        panic!("Error: {}", tt.len());
    });
    let transaction_hash = Sha256::digest(transaction_hash);
    // let transaction_hash22 = Sha256::digest(transaction_hash);

    let witness = t.vin[idx].witness.clone();
    let binding = witness[0].clone();

    let signature_bytes = hex::decode(binding).unwrap();

    // signature into der encoded byte slice
    let signature = match Signature::from_der_lax(&signature_bytes) {
        Ok(signature) => signature,
        Err(e) => panic!("Error: {:?}", e),
    };
    //  println!("{:?}",signature);

    let pub_key = witness.last().unwrap();
    let pub_key = hex::decode(pub_key).unwrap();

    let pub_key = PublicKey::from_slice(&pub_key).unwrap();

    let secp = Secp256k1::verification_only();

    let message = Message::from_digest_slice(&Sha256::digest(transaction_hash)).unwrap();

    // println!("{}   {}",message ,pub_key);
    secp.verify_ecdsa(&message, &signature, &pub_key).is_ok()
}
