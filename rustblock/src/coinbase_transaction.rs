use super::utiles::{
    convert_to_4bytes, convert_to_8bytes, double_sha256, int_to_varint, merkle_root,
};

use crate::{transacton_struct::*, utiles::reverse_bytes};

pub fn construct_coinbase_transaction(
    block_reward: u64,
    transaction_fees: u64,
    transactions: Vec<Transaction>,
) -> Transaction {
    let mut wit = Vec::new();
    wit.push("0000000000000000000000000000000000000000000000000000000000000000".to_string());
    let mut coinbase_inputs = Vec::new();
    let coinbase_input = TransactionInput {
        txid: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        vout: 4294967295, // Coinbase transaction does not reference any previous output
        prevout: PrevOut {
            scriptpubkey: String::from(""),
            scriptpubkey_asm: String::from(""),
            scriptpubkey_type: String::from(""),
            scriptpubkey_address: String::from(""),
            value: 0,
        },
        scriptsig: String::from(
            "03233708184d696e656420627920416e74506f6f6c373946205b8160a4256c0000946e0100",
        ),
        scriptsig_asm: String::from(""),
        witness: wit,
        inner_redeemscript_asm: String::from(""),
        is_coinbase: true,
        sequence: 0xFFFFFFFF,
    };
    coinbase_inputs.push(coinbase_input);

    let mut coinbase_outputs = Vec::new();
    let miner_output = TransactionOutput {
        scriptpubkey:String::from("41047eda6bd04fb27cab6e7c28c99b94977f073e912f25d1ff7165d9c95cd9bbe6da7e7ad7f2acb09e0ced91705f7616af53bee51a238b7dc527f2be0aa60469d140ac"), // Use the generated script 
        scriptpubkey_asm: String::from(""), 
        scriptpubkey_type: String::from("p2pkh"),
        scriptpubkey_address: String::from(""),
        value: block_reward + transaction_fees
    };
    let miner_output2 = TransactionOutput {
        scriptpubkey: calculate_witness_commitment(transactions).to_string(),
        scriptpubkey_asm: String::from(""),
        scriptpubkey_type: String::from(""),
        scriptpubkey_address: String::from(""),
        value: 0,
    };

    coinbase_outputs.push(miner_output);
    coinbase_outputs.push(miner_output2);

    let coinbase_transaction = Transaction {
        version: 1,
        locktime: 0,
        vin: coinbase_inputs,
        vout: coinbase_outputs,
        fees: 0,
    };

    coinbase_transaction
}

fn calculate_witness_commitment(transactions: Vec<Transaction>) -> String {
    let mut wtxids: Vec<String> = Vec::new();
    // witness for coinbase  transaction
    wtxids.push("0000000000000000000000000000000000000000000000000000000000000000".to_string());
    for t in transactions {
        let wtxid = wtxid_data(t);
        wtxids.push(wtxid);
    }
    // sort the wtxids
    // wtxids.sort();
    let mut merkle_root = merkle_root(wtxids);

    // witness reserved value of coinbase transaction
    merkle_root.push_str("0000000000000000000000000000000000000000000000000000000000000000");

    let witness_commitment = double_sha256(merkle_root);
    // fix prefix for witness commitment "6a24aa21a9ed"
    let mut wit_new = String::from("6a24aa21a9ed".to_string());
    wit_new.push_str(&witness_commitment);
    wit_new
}

fn wtxid_data(t: Transaction) -> String {
    let mut transaction_data = String::new();

    //  1 byte input count in hexadicimal number, convert to hexadicimal
    let input_count = t.vin.len();

    transaction_data.push_str(&int_to_varint(input_count as u64));

    let mut is_segwit = false;

    let mut witness = String::new();

    for i in 0..t.vin.len() {
        // 32 bytes prevout hash txid as little endian, convert to little endian

        transaction_data.push_str(&reverse_bytes(t.vin[i].txid.clone()));

        // 4 bytes prevout index little endian

        let vout = t.vin[i].vout;
        transaction_data.push_str(&convert_to_4bytes(vout));

        // 1 byte scriptpubkey length\
        let pub_key_len = t.vin[i].scriptsig.len() / 2;
        transaction_data.push_str(int_to_varint(pub_key_len as u64).as_str());
        //pub key
        transaction_data.push_str(&t.vin[i].scriptsig);

        // 4 bytes sequence, is always ffffffff
        transaction_data.push_str(convert_to_4bytes(t.vin[i].sequence).as_str());

        if t.vin[i].prevout.scriptpubkey_type == "p2pkh".to_string()
            || t.vin[i].prevout.scriptpubkey_type == "p2sh".to_string()
        {
            witness.push_str("00");
        } else {
            is_segwit = true;
            // number of witness
            let witness_cnt = t.vin[i].witness.len();
            witness.push_str(&int_to_varint(witness_cnt as u64));
            for j in 0..t.vin[i].witness.len() {
                let pub_key_len = t.vin[i].witness[j].len() / 2;
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
        transaction_data.push_str(&convert_to_8bytes(amount as u64));
        // 1 byte scriptPubKey length
        let scriptpubkey_len = t.vout[i].scriptpubkey.len() / 2;
        transaction_data.push_str(int_to_varint(scriptpubkey_len as u64).as_str());
        // scriptPubKey
        transaction_data.push_str(&t.vout[i].scriptpubkey);
    }

    if is_segwit {
        transaction_data.push_str(&witness);
        transaction_data.insert_str(0, "0001");
    }
    transaction_data.insert_str(0, &convert_to_4bytes(t.version));
    transaction_data.push_str(&convert_to_4bytes(t.locktime));

    // Calculate the hash of the transaction data

    let wtxid = double_sha256(transaction_data);
    wtxid
}
