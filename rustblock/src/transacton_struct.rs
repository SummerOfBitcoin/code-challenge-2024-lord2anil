use derivative::Derivative;
use serde::Deserialize;

#[derive(Derivative, Debug, Deserialize, Default, Clone)]
pub struct Transaction {
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<TransactionInput>,
    pub vout: Vec<TransactionOutput>,
    // fees
    #[serde(default)]
    pub fees: u64,
    // weight
    #[serde(default)]
    pub weight:u64
}

#[derive(Derivative, Debug, Deserialize, Default, Clone)]
pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
    pub prevout: PrevOut,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    #[serde(default)]
    pub witness: Vec<String>,
    #[serde(default)]
    pub inner_redeemscript_asm: String,
    pub is_coinbase: bool,
    pub sequence: u32,
}

#[derive(Derivative, Debug, Deserialize, Default, Clone)]
pub struct PrevOut {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    #[serde(default)]
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Derivative, Debug, Deserialize, Default, Clone)]
pub struct TransactionOutput {
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    #[serde(default)]
    pub scriptpubkey_address: String,
    pub scriptpubkey: String,
    pub value: u64,
}
