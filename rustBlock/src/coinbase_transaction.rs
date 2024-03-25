



use crate::transacton_struct::*;

pub fn construct_coinbase_transaction(block_reward: u64, transaction_fees: u64) -> Transaction {
    let mut coinbase_inputs = Vec::new();
    let coinbase_input = TransactionInput {
        txid: String::from(""),
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
    
    let mut coinbase_outputs = Vec::new();
    let miner_output = TransactionOutput {
        scriptpubkey: String::from(""),
        scriptpubkey_asm: String::from(""),
        scriptpubkey_type: String::from(""),
        scriptpubkey_address: String::from(""), // Address of the miner
        value: block_reward,
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
