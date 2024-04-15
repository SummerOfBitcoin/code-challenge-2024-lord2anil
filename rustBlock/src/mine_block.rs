use num::Num;
use sha2::{Digest, Sha256};
use hex;
use crate::transacton_struct::*;
use crate::assemble_block::*;
use num::{BigUint, FromPrimitive};


fn convert_to_4bytes(num:u32)->String{

    let mut bytes = vec![];
    bytes.extend_from_slice(&(num as u32).to_le_bytes());
 
     // Convert the bytes to a hexadecimal string
     let hex_string = hex::encode(&bytes);
        hex_string
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





pub fn mine_block(mut block:Block, target:String)-> Block{

    let new_block = block.clone();
    let header_data = format!(
        "{}{}{}{}{}",
        reverse_bytes(new_block.version),
        reverse_bytes(new_block.prev_block_hash),
        reverse_bytes(new_block.merkle_root),
        reverse_bytes(new_block.timestamp),
        reverse_bytes(new_block.bits)
    );
    let mut nonce = 0;

    loop {
        // Hash the block header
        let attempt = format!("{}{}", header_data, reverse_bytes(convert_to_4bytes(nonce)));
        let result = reverse_bytes(double_sha256(attempt));

        // // Show result
        // println!("{}: {}", nonce, result);
        

        // Break if the hash is below the target
        if BigUint::from_str_radix(&result, 16).unwrap() < BigUint::from_str_radix(&target, 16).unwrap(){
            break;
        }
       

        nonce += 1;
    }


    block.nonce=nonce;
    return block


    

    
}