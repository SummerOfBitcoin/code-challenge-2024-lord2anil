



use crate::{reverse_bytes, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

use super::utiles::{convert_to_4bytes, convert_to_8bytes, int_to_varint,double_sha256,merkle_root};





#[derive(Debug,Clone)]
pub struct Block {
    pub version: String,
    pub prev_block_hash: String,  // Assume hashes are represented as strings
    pub merkle_root: String,
    pub timestamp: String,
    pub bits: String,
    pub nonce: u32,
    pub transactions: Vec<Transaction>,
}


pub fn assemble_block(transactions: Vec<Transaction>) ->Block {
    // Calculate the merkle root of the transactions
    let  txids = calculate_txid(&transactions);

  

    let merkle_root = merkle_root(txids.clone());

 

    // assemble the block
    let block = Block {
        version: "04000000".to_string(),
        prev_block_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        merkle_root: merkle_root,
        // unix timestamp
        timestamp: convert_to_4bytes(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32),

        bits: reverse_bytes( "1f00ffff".to_string()),
        nonce: 0,
        transactions: transactions,
    };

    block






  
}



fn txid_data(t: Transaction) -> String {
    
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




// Function to calculate Merkle root of transactions
 fn calculate_txid(transactions: &[Transaction]) -> Vec<String> {
    let mut txids = vec![];

    for t in transactions {
       
        let txid = txid_data(t.clone());
        
        txids.push( txid);
    }
    txids

}

