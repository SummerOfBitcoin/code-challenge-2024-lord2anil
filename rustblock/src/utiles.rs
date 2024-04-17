


use sha2::{Digest, Sha256};

use crate::Transaction;
use std::io::Write;

use std::fs::File;




pub fn double_sha256(data:String) -> String {
    // Convert the hexadecimal string to a byte array.
    let bytes = hex::decode(data).unwrap();

    // Calculate the SHA-256 hash of the byte array.
    let hash = Sha256::digest(&bytes);

    // Calculate the double SHA-256 hash of the byte array.
    let double_hash = Sha256::digest(&hash);

    // Convert the hash to a hexadecimal string.
    let hex_string = hex::encode(double_hash);

    hex_string

}

pub fn merkle_root(txids: Vec<String>) -> String {
    // Exit Condition: Stop recursion when we have one hash result left
    if txids.len() == 1 {
        // Convert the result to a string and return it
        return txids[0].clone();
    }

    // Keep an array of results
    let mut result = Vec::new();

    // Split up array of hashes in to pairs
    for chunk in txids.chunks(2) {
        let concat = match chunk.len() {
            2 => chunk[0].clone() + &chunk[1],
            1 => chunk[0].clone() + &chunk[0], // Concatenate with itself if there is no pair
            _ => panic!("Unexpected length"),
        };

        // Hash the concatenated pair and add to results array
        result.push(double_sha256(concat));
    }

    // Recursion: Do the same thing again for these results
    merkle_root(result)
}

pub fn convert_to_4bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u32).to_le_bytes());
 
     // Convert the bytes to a hexadecimal string
     let hex_string = hex::encode(&bytes);
        hex_string
}
pub fn convert_to_8bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u64).to_le_bytes());
 
     // Convert the bytes to a hexadecimal string
     let hex_string = hex::encode(&bytes);
        hex_string
}

pub fn int_to_varint(n: u64) -> String {
    
    if n <= 252 {  // 0xFC
        return hex::encode (vec![n as u8]);
    } else if n <= 65535 {  // 0xFFFF
        let mut bytes = vec![0xFD];
        bytes.extend_from_slice(&(n as u16).to_le_bytes());
        return  hex::encode (bytes);
    } else if n <= 4294967295 {  // 0xFFFFFFFF
        let mut bytes = vec![0xFE];
        bytes.extend_from_slice(&(n as u32).to_le_bytes());
        return hex::encode (bytes);
    } else {
        let mut bytes = vec![0xFF];
        bytes.extend_from_slice(&n.to_le_bytes());
        return hex::encode (bytes);
    }
}

pub fn reverse_bytes(hex_string:String)->String{
    // println!("{}",hex_string);
    let bytes = hex::decode(hex_string).unwrap();

    // Reverse the order of the bytes.
    let reversed_bytes = bytes.iter().rev().cloned().collect::<Vec<u8>>();

    // Convert the reversed bytes to a string.
    let reversed_hex_string = hex::encode(reversed_bytes);
    reversed_hex_string

}



pub fn serialize_coinbase_transaction(t: &Transaction) -> String {

    let mut serialized_tx = String::new();
    // Serialize the version
    serialized_tx.push_str(&convert_to_4bytes(t.version));
    //marker and flag
    serialized_tx.push_str("0001");
    // Serialize the number of inputs
    serialized_tx.push_str(&int_to_varint(t.vin.len() as u64));
    // Serialize the inputs
    for i in 0..t.vin.len() {
      
            // 32 bytes prevout hash txid as little endian, convert to little endian
            // Decode the hex string to a byte array.
            let hex_string = t.vin[i].txid.clone();
            let bytes = hex::decode(hex_string).unwrap();

            // Reverse the order of the bytes.
            let reversed_bytes = bytes.iter().rev().cloned().collect::<Vec<u8>>();

            // Convert the reversed bytes to a string.
            let reversed_hex_string = hex::encode(reversed_bytes);
            serialized_tx.push_str(&reversed_hex_string);

            // 4 bytes prevout index little endian
            

            let vout = t.vin[i].vout;
            serialized_tx.push_str(
                &convert_to_4bytes(vout)
            );

            // 1 byte scriptsig length\
            let pub_key_len =t.vin[i].scriptsig.len() / 2;
            // let pub_key_len =t.vin[i].scriptsig.len() / 2;
            serialized_tx.push_str(int_to_varint(pub_key_len as u64).as_str());
            //pub key
            serialized_tx.push_str(&t.vin[i].scriptsig);

            // 4 bytes sequence, is always ffffffff
            serialized_tx.push_str(convert_to_4bytes(t.vin[i].sequence).as_str());

       
    }
    // for output

    let output_count = t.vout.len();
    serialized_tx.push_str(&int_to_varint(output_count as u64));
    for i in 0..t.vout.len() {
        // 8 bytes amount in little endian
        let amount = t.vout[i].value;
        serialized_tx.push_str(
            &convert_to_8bytes(amount as u32)
        );
        // 1 byte scriptPubKey length
        let scriptpubkey_len = t.vout[i].scriptpubkey.len() / 2;
        serialized_tx.push_str(int_to_varint(scriptpubkey_len as u64).as_str());
        // scriptPubKey
        serialized_tx.push_str(&t.vout[i].scriptpubkey);
        // println!("{}",t.vout[i].scriptpubkey);
    }
    // witness for coinbase
    serialized_tx.push_str("01200000000000000000000000000000000000000000000000000000000000000000");
    serialized_tx.push_str(&convert_to_4bytes(t.locktime));
    
        
    serialized_tx

  
}





pub fn txid_data(t: Transaction) -> String {
    
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
            let pub_key_len =t.vin[i].scriptsig.len() / 2;
            transaction_data.push_str(int_to_varint(pub_key_len as u64).as_str());
            //pub key
            transaction_data.push_str(&t.vin[i].scriptsig);

            // 4 bytes sequence, is always ffffffff
            transaction_data.push_str(convert_to_4bytes(t.vin[i].sequence).as_str());

            

       
    }
    // for output

   
    let output_count = t.vout.len();
    transaction_data.push_str(&int_to_varint(output_count as u64));
    for i in 0..t.vout.len() {
        // 8 bytes amount in little endian
        let amount = t.vout[i].value;
        transaction_data.push_str(
            &convert_to_8bytes(amount as u32)
        );
        // 1 byte scriptPubKey length
        let scriptpubkey_len = t.vout[i].scriptpubkey.len() / 2;
        transaction_data.push_str(int_to_varint(scriptpubkey_len as u64).as_str());
        // scriptPubKey
        transaction_data.push_str(&t.vout[i].scriptpubkey);
    }
    
    
   
    transaction_data.push_str(&convert_to_4bytes(t.locktime));
    // println!("{}",transaction_data);

    // Calculate the hash of the transaction data
    let txid = double_sha256(transaction_data);
    txid
    
}



pub // // Function to write block data to output.txt file
fn write_to_output_file(header:String, coinbase_txid: &str, transaction_txids: Vec<String>) {
    let mut file = match File::create("../output.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("Error creating output.txt file");
            return;
        }
    };

    // Write block header to file
    writeln!(file, "{}", header).expect("Error writing to file");

    // Write serialized coinbase transaction to file
    writeln!(file, "{}", coinbase_txid).expect("Error writing to file");

    // Write transaction IDs of mined transactions to file

    for txid in transaction_txids {
        writeln!(file, "{}", txid).expect("Error writing to file");
    }
}


