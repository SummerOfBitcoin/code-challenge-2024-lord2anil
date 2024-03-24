use serde::{Deserialize};


#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct Transaction {
    version: u32,
    locktime: u32,
    vin: Vec<TransactionInput>,
    vout: Vec<TransactionOutput>,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
struct TransactionInput {
    txid: String,
    vout: u32,
    prevout: PrevOut,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Vec<String>,
    is_coinbase: bool,
    sequence: u32,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
struct PrevOut {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: u64,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
struct TransactionOutput {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: u64,
}
