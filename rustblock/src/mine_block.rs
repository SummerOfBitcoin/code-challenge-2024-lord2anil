use num::Num;


use crate::assemble_block::*;
use num::BigUint;

use super::utiles::{convert_to_4bytes, double_sha256, reverse_bytes};







pub fn mine_block(mut block:Block, target:String)-> Block{

    let new_block = block.clone();
    let header_data = format!(
        "{}{}{}{}{}",
        (new_block.version),
        (new_block.prev_block_hash),
        (new_block.merkle_root),
        (new_block.timestamp),
        (new_block.bits)
    );
    let mut nonce = 0;

    loop {
        // Hash the block header
        let attempt = format!("{}{}", header_data, (convert_to_4bytes(nonce)));
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