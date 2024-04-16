

use sha2::{Digest, Sha256};

use crate::transacton_struct::*;

pub fn construct_coinbase_transaction(
    block_reward: u64, 
    transaction_fees: u64,
    miner_public_key_hash: &str,   
) -> Transaction {

    let mut wit=Vec::new();
    wit.push("0000000000000000000000000000000000000000000000000000000000000000".to_string());
    let mut coinbase_inputs = Vec::new();
    let coinbase_input = TransactionInput {
        txid: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        vout: 4294967295, // Coinbase transaction does not reference any previous output
        prevout: PrevOut {
            scriptpubkey: String::from(""), // Arbitrary data for the coinbase input
            scriptpubkey_asm: String::from(""),
            scriptpubkey_type: String::from(""),
            scriptpubkey_address: String::from(""),
            value: 0, // Value is typically set to 0 for coinbase inputs
        },
        scriptsig: String::from("03233708184d696e656420627920416e74506f6f6c373946205b8160a4256c0000946e0100"), // Empty script signature for coinbase input
        scriptsig_asm: String::from(""),
        witness: wit,
        is_coinbase: true,
        sequence: 0xFFFFFFFF, // Set a high sequence number for coinbase input
    };
    coinbase_inputs.push(coinbase_input);
    
    let miner_script_pubkey = create_p2pkh_script(miner_public_key_hash); 

    let mut coinbase_outputs = Vec::new();
    let miner_output = TransactionOutput {
        scriptpubkey:String::from("41047eda6bd04fb27cab6e7c28c99b94977f073e912f25d1ff7165d9c95cd9bbe6da7e7ad7f2acb09e0ced91705f7616af53bee51a238b7dc527f2be0aa60469d140ac"), // Use the generated script 
        scriptpubkey_asm: String::from(""), // You would need to fill this if required
        scriptpubkey_type: String::from("p2pkh"),
        scriptpubkey_address: String::from(""), // Derive if needed
        value: block_reward + transaction_fees, // Include transaction fees
    };
    let miner_output2 = TransactionOutput {
        scriptpubkey:String::from("faa194df59043645ba0f58aad74bfd5693fa497093174d12a4bb3b0574a878db"), // Use the generated script 
        scriptpubkey_asm: String::from(""), // You would need to fill this if required
        scriptpubkey_type: String::from(""),
        scriptpubkey_address: String::from(""), // Derive if needed
        value: 0, // Include transaction fees
    };
    coinbase_outputs.push(miner_output);
    coinbase_outputs.push(miner_output2);
    
    // Additional outputs for transaction fees can be added here
    
    let coinbase_transaction = Transaction {
        version: 1,
        locktime: 0,
        vin: coinbase_inputs,
        vout: coinbase_outputs,
    };
    
    coinbase_transaction
}
fn create_p2pkh_script(public_key_hash: &str) -> String {
    // Placeholder - you would need the logic to generate a valid P2PKH script 
    // based on the public key hash. This involves specific opcodes and formatting
    format!("OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG", public_key_hash) 
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
fn convert_to_4bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u32).to_le_bytes());
 
     // Convert the bytes to a hexadecimal string
     let hex_string = hex::encode(&bytes);
        hex_string
}
fn convert_to_8bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u64).to_le_bytes());
 
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
