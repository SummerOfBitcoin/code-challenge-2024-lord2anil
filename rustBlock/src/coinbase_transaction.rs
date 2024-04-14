



use crate::transacton_struct::*;

pub fn construct_coinbase_transaction(
    block_reward: u64, 
    transaction_fees: u64,
    miner_public_key_hash: &str,   
) -> Transaction {
    let mut coinbase_inputs = Vec::new();
    let coinbase_input = TransactionInput {
        txid: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        vout: 0, // Coinbase transaction does not reference any previous output
        prevout: PrevOut {
            scriptpubkey: String::from(""), // Arbitrary data for the coinbase input
            scriptpubkey_asm: String::from(""),
            scriptpubkey_type: String::from(""),
            scriptpubkey_address: String::from(""),
            value: 0, // Value is typically set to 0 for coinbase inputs
        },
        scriptsig: String::from(""), // Empty script signature for coinbase input
        scriptsig_asm: String::from(""),
        witness: Vec::new(),
        is_coinbase: true,
        sequence: 0xFFFFFFFF, // Set a high sequence number for coinbase input
    };
    coinbase_inputs.push(coinbase_input);
    
    let miner_script_pubkey = create_p2pkh_script(miner_public_key_hash); 

    let mut coinbase_outputs = Vec::new();
    let miner_output = TransactionOutput {
        scriptpubkey:String::from(""), // Use the generated script 
        scriptpubkey_asm: String::from(""), // You would need to fill this if required
        scriptpubkey_type: String::from("p2pkh"),
        scriptpubkey_address: String::from(""), // Derive if needed
        value: block_reward + transaction_fees, // Include transaction fees
    };
    coinbase_outputs.push(miner_output);
    
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