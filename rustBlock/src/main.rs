
use serde::{Deserialize};
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, Read};
use std::path::Path;
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
use std::path::{ PathBuf};

fn serialize_block_header(block: &Block) -> String {
    // You'll need to implement this based on your Block structure's format

    // println!("{}",block.version);
    // println!("{}",block.prev_block_hash);
    // println!("{}",block.merkle_root);
    // println!("{}",block.timestamp);
    // println!("{}",block.bits);
    println!("{}",block.nonce);



     format!(
         "{}{}{}{}{}{}",
         (block.version.clone()),
         (block.prev_block_hash.clone()),
         (block.merkle_root.clone()),
         (block.timestamp.clone()),
         (block.bits.clone()),
        ( convert_to_4bytes(block.nonce))
     )

 }
 fn convert_to_4bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u32).to_le_bytes());
 
     // Convert the bytes to a hexadecimal string
     let hex_string = hex::encode(&bytes);
        hex_string
}

fn int_to_varint(n: u64) -> String {
    
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
fn convert_to_8bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u64).to_le_bytes());
 
     // Convert the bytes to a hexadecimal string
     let hex_string = hex::encode(&bytes);
        hex_string
}

 fn serialize_coinbase_transaction(t: &Transaction) -> String {

    let mut serialized_tx = String::new();
    // Serialize the version
    serialized_tx.push_str(&convert_to_4bytes(t.version));
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

            // 1 byte scriptpubkey length\
            let pub_key_len =t.vin[i].prevout.scriptpubkey.len() / 2;
            serialized_tx.push_str(int_to_varint(pub_key_len as u64).as_str());
            //pub key
            serialized_tx.push_str(&t.vin[i].prevout.scriptpubkey);

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
    }
    serialized_tx.push_str(&convert_to_4bytes(t.locktime));
    
        serialized_tx.push_str("01000000");
    serialized_tx

  
}
 
fn main() {
    

    let folder_path = "../mempool";

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
  

    // let sample_file_name="../t.txt";
    
    
    // // Read the file names from t.txt, line by line
    // let file = File::open(sample_file_name).unwrap();
    // let reader = std::io::BufReader::new(file);
    // let mut valid_file_names: Vec<PathBuf> = Vec::new();

    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let pp = format!("../mempool/{}", line);
    //     let path = PathBuf::from(pp); // Construct PathBuf directly
    //     valid_file_names.push(path);
    // }
    // valid_file_names.pop();
    // let mut p=true;
    // for path in valid_file_names {
    //    if p==true {
    //        p=false;
    //        continue;
    //    }
    //   x=x+1;
    //     let mut file = File::open(path).unwrap();
    //     let mut data = String::new();
    //     file.read_to_string(&mut data).unwrap();
        
    //     let transaction: Transaction = match serde_json::from_str::<Transaction>(&data) {
            
    //         Ok(result) => result,
    //         Err(_e) => {
                
    //         //    println!("{}",e);
    //             continue;
    //         }

           
    //     };
        
    //     transactions.push(transaction);
    // }
   
    transactions= validate_transactions(&transactions).clone();
    let mut ttt=transactions[0].clone();
    transactions.clear();
    transactions.push(ttt);
    // println!("{:?} {x}", transactions.len());


 let coinbase_transaction: Transaction = construct_coinbase_transaction(6, 1,"03178e5f7ba2f41f449ce90ba0635bad19bbb17d7e634ed50f96c4f956e704d188"); // Example block reward and transaction fees
    transactions.insert(0, coinbase_transaction);
    let block = assemble_block(transactions);
   

  

  let difficulty_target = "0000ffff00000000000000000000000000000000000000000000000000000000";

    // Mine the block
   let mined_block = mine_block(block, difficulty_target.to_string());


    // Print the mined block
    // println!("hello this {:#?}", mined_block.nonce);

    // generate the block header
    let block_header = serialize_block_header(&mined_block);


    // Serialize the coinbase transaction
    let coinbase_tx = serialize_coinbase_transaction(&mined_block.transactions[0]);
    let txids = calculate_txid(&mined_block.transactions);



     println!("{}",block_header);
    // Write block data to output.txt file
   write_to_output_file(block_header, "f23dfsfwf", txids);
}



fn double_sha256(data:String) -> String {
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
fn reverse_bytes(hex_string:String)->String{
    // println!("{}",hex_string);
    let bytes = hex::decode(hex_string).unwrap();

    // Reverse the order of the bytes.
    let reversed_bytes = bytes.iter().rev().cloned().collect::<Vec<u8>>();

    // Convert the reversed bytes to a string.
    let reversed_hex_string = hex::encode(reversed_bytes);
    reversed_hex_string

}

fn txid_data(t: Transaction) -> String {
    
    // 4 bits version, in little endian
    // transaction_data.push_str(
    //     &convert_to_4bytes(t.version)
    // );
    

 

    let mut transaction_data = String::new();
     
    //  1 byte input count in hexadicimal number, convert to hexadicimal
    let input_count = t.vin.len();

    
    transaction_data.push_str(&int_to_varint(input_count as u64));
    // if len of vin is greater 255, then use verint
    
   
     let mut  is_segwit = false;

     let mut witness = String::new();

   
    for i in 0..t.vin.len() {
       
        if t.vin[i].prevout.scriptpubkey_type=="p2pkh".to_string() {
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

            witness.push_str("00");

        } else {
            is_segwit = true;
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


            // number of witness
            let mut witness_cnt=t.vin[i].witness.len();
            witness.push_str(&int_to_varint(witness_cnt as u64));
            for j in 0..t.vin[i].witness.len() {
                let pub_key_len =t.vin[i].witness[j].len() / 2;
                witness.push_str(int_to_varint(pub_key_len as u64).as_str());
                witness.push_str(&t.vin[i].witness[j]);
            }
            
        }
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
    
    
    if is_segwit {
        transaction_data.push_str(&witness);
        
        
    }
    transaction_data.insert_str(0, &convert_to_4bytes(t.version));
    transaction_data.push_str(&convert_to_4bytes(t.locktime));
    // println!("{}",transaction_data);


    // Calculate the hash of the transaction data
    

       
    let txid = double_sha256(transaction_data);
    txid
    
}


use sha2::{Digest, Sha256};

// Function to calculate Merkle root of transactions
 fn calculate_txid(transactions: &[Transaction]) -> Vec<String> {
    let mut txids = vec![];
      
    for t in transactions {
      
       
        let txid = txid_data(t.clone());
        // println!("{}",txid);
        
        txids.push(reverse_bytes(txid));
    }
    txids

}
   

use std::io::Write;

// // Function to write block data to output.txt file
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




