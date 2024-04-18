
use ripemd::Ripemd160;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};

use crate::Transaction;



use crate::utiles::{convert_to_4bytes, convert_to_8bytes, int_to_varint};


pub fn p2pkh_validate(t: &Transaction, idx: usize) -> bool {
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
pub fn p2pkh_verify_signature(t: Transaction, idx: usize) -> bool {
    let mut transaction_data = String::new();
    // 4 bits version, in little endian
    transaction_data.push_str(
        &convert_to_4bytes(t.version)
    );
    
 
    //  1 byte input count in hexadicimal number, convert to hexadicimal
    let input_count = t.vin.len();

    
    transaction_data.push_str(&int_to_varint(input_count as u64));
    // if len of vin is greater 255, then use verint
    
   
   
    for i in 0..t.vin.len() {
        if i == idx {
            // 32 bytes prevout hash txid as little endian, convert to little endian
            // Decode the hex string to a byte array.
            let hex_string = t.vin[i].txid.clone();
            let bytes = hex::decode(hex_string).unwrap();

            // Reverse the order of the bytes.
            let reversed_bytes = bytes.iter().rev().cloned().collect::<Vec<u8>>();

            // Convert the reversed bytes to a string.
            let reversed_hex_string = hex::encode(reversed_bytes);
            transaction_data.push_str(&reversed_hex_string);

            // 4 bytes prevout index little endian
            

            let vout = t.vin[i].vout;
            transaction_data.push_str(
                &convert_to_4bytes(vout)
            );

            // 1 byte scriptpubkey length\
            let pub_key_len =t.vin[i].prevout.scriptpubkey.len() / 2;
            transaction_data.push_str(int_to_varint(pub_key_len as u64).as_str());
            //pub key
            transaction_data.push_str(&t.vin[i].prevout.scriptpubkey);

            // 4 bytes sequence, is always ffffffff
            transaction_data.push_str(convert_to_4bytes(t.vin[i].sequence).as_str());

        } else {
            // 32 bytes prevout hash txid
            let hex_string = t.vin[i].txid.clone();
            let bytes = hex::decode(hex_string).unwrap();

            // Reverse the order of the bytes.
            let reversed_bytes = bytes.iter().rev().cloned().collect::<Vec<u8>>();

             // Convert the reversed bytes to a string.
             let reversed_hex_string = hex::encode(reversed_bytes);
             transaction_data.push_str(&reversed_hex_string);
            // 4 bytes prevout index
            let vout = t.vin[i].vout;
            transaction_data.push_str(
                &convert_to_4bytes(vout)
            );
            // 1 byte scriptSig length
            // need to remove the scriptsig , so length is 0
            transaction_data.push_str("00");
            // 4 bytes sequence, is always ffffffff
            transaction_data.push_str(convert_to_4bytes(t.vin[i].sequence).as_str());
            
        }
    }
    // for output

   
    let output_count = t.vout.len();
    transaction_data.push_str(&int_to_varint(output_count as u64));
    for i in 0..t.vout.len() {
        // 8 bytes amount in little endian
        let amount = t.vout[i].value;
        transaction_data.push_str(
            &convert_to_8bytes(amount as u64)
        );
        // 1 byte scriptPubKey length
        let scriptpubkey_len = t.vout[i].scriptpubkey.len() / 2;
        transaction_data.push_str(int_to_varint(scriptpubkey_len as u64).as_str());
        // scriptPubKey
        transaction_data.push_str(&t.vout[i].scriptpubkey);
    }
    transaction_data.push_str(&convert_to_4bytes(t.locktime));
    
        transaction_data.push_str("01000000");
        
       

        // sha256 hash of transaction data
    //   println!("{}",transaction_data);

        if transaction_data.len() % 2 != 0 {
            println!("hhekijer");
            transaction_data = format!("0{}", transaction_data) ;
        }
        let tt=transaction_data.clone();
        // println!("{}",transaction_data);
        let transaction_hash = hex::decode(transaction_data).unwrap_or_else(|_e| {

         panic!("Error: {}", tt.len());
        });
        let transaction_hash = Sha256::digest(transaction_hash);
        let transaction_hash22 = Sha256::digest(transaction_hash);
       

        let scriptsig_asm1 = t.vin[idx].scriptsig_asm.clone();
    let binding = scriptsig_asm1.split(" ").collect::<Vec<&str>>()[1];
   

    let signature_bytes = hex::decode(binding).unwrap(); // Replace hex with your encoding format if different

    // signature into der encoded byte slice
    let signature = match Signature::from_der_lax(&signature_bytes) {
        Ok(signature) => signature,
        Err(e) => panic!("Error: {:?}", e),
    };
    //  println!("{:?}",signature);
    let binding = t.vin[idx].scriptsig_asm.split(" ").collect::<Vec<&str>>();
    let pub_key = binding.last().unwrap();
    let pub_key = hex::decode(pub_key).unwrap();
    let pub_key = PublicKey::from_slice(&pub_key).unwrap();
    let secp = Secp256k1::verification_only();
    
    let message = Message::from_digest_slice(&Sha256::digest(transaction_hash)).unwrap();

    // println!("{}   {}",message ,pub_key);
    secp.verify_ecdsa(&message, &signature, &pub_key).is_ok()
    // println!("{:?}",pub_key);
}