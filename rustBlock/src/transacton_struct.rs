use serde::{Deserialize};


#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct Transaction {
    pub version: u32,
    pub  locktime: u32,
    pub vin: Vec<TransactionInput>,
    pub vout: Vec<TransactionOutput>,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct TransactionInput {
    pub  txid: String,
    pub  vout: u32,
    pub  prevout: PrevOut,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub witness: Vec<String>,
    pub is_coinbase: bool,
    pub sequence: u32,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct PrevOut {
    pub  scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct TransactionOutput {
   pub scriptpubkey_asm: String,
   pub scriptpubkey_type: String,
   pub scriptpubkey_address: String,
   pub scriptpubkey: String,
   pub value: u64,
}
