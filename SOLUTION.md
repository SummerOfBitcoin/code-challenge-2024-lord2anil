


## Design Approach
The design approach for this problem is to create a block construction program that can validate transactions and mine them into a block. The key concepts of creating a valid block include:

- **Transaction Validation:** The program should be able to validate transactions based on the given set of transactions. This involves checking the transaction inputs, outputs, and signatures to ensure that they are valid. And to get the high score i sort all the transactions based on the weight;

- **coinbase Transaction:** The coinbase transaction is the first transaction in a block. It is a special type of transaction that does not have any inputs and is used to collect the block reward. 

- **Assemble Block:** The program should be able to assemble the block by including valid transactions in the block. The block should  includes the coinbase transaction.

- **Mining the Block:** The program should be able to mine the block by finding a hash that is less than the difficulty target. This involves calculating the hash of the block header and adjusting the nonce value until a valid hash is found.

- **Output:** The program should generate an output file named `output.txt` that follows a specific format. The output file should contain the block header, the serialized coinbase transaction, and the transaction IDs of the transactions mined in the block.

## Implementation Details

### The implementation of the block construction program involves the following steps:

#### Frequentlly used functions
The `rustblock/src/utiles.rs` file contains the all functions that are used in the program. 
Some frewuentlly used functions are as follows:
- **double_sha256:** The function calculates the double SHA-256 hash of the input data.
- **reverse_bytes:** The function reverses the bytes of the input data.
- **convert_to_4bytes:** The function converts the input data to a 4-byte little-endian format.
- **convert_to_8bytes:** The function converts the input data to a 8-byte little-endian format.
- **merkle_root:** The function calculates the merkle root of the input data.


1. Struct `Transaction` to represent a transaction. The structure of the transaction is as follows:
```
pub struct Transaction {
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<TransactionInput>,
    pub vout: Vec<TransactionOutput>,
    pub fees: u64,
    pub weight:u64
}
```
The more details of the transaction can be found in the `rustblock/src/transacton_struct.rs` file.

2. Read the tranasactions from the mempool and parse the transactions. The mempool folder contains the json files  ,each json file represent a Transactions. The program json file and parses the transactions into a vector of `Transaction` objects.
i have used the `serde_json` crate to parse the json file.

3. Validate the transactions by checking the inputs, outputs, and signatures of the transactions. The program validates the transactions by checking the inputs, outputs, and signatures of the transactions. 
There are around 8131 transactions in the mempool folder. For tha validation i verify only the 2 types of transactions:
- **P2PKH:** Pay-to-Public-Key-Hash (P2PKH) is the most common type of transaction in Bitcoin. There are aroud 441 transactions of this type that are valid.
- **P2WPKH:** Pay-to-Witness-Public-Key-Hash (P2WPKH) is a type of transaction that uses the Segregated Witness (SegWit) format. There are around 3100 transactions of this type that are valid.

 The validation of the transactions is done in the `rustblock/src/validate_transactions.rs` file.
 The folder `rustblock/src/validation_scripts` contains the validation logic  for the P2PKH and P2WPKH transactions.
 the structure of the validation script folder is as follows:
 ```
  rustblock/src/validation_scripts  
    ├── p2pkh.rs  //contains the validation logic for P2PKH transactions.
    └── p2wpkh.rs  //contains the validation logic for P2WPKH transactions.
```
So the validate_transactions function in the `rustblock/src/validate_transactions.rs` file check the transaction is valid of not and if it is valid then push it to the valid transaction vector.
```rust
pub fn validate_transactions(transactions: &[Transaction]) -> Vec<Transaction> {
    let mut valid_transactions: Vec<Transaction> = Vec::new();
    for transaction in transactions {
        if is_valid_transaction(transaction) {
            valid_transactions.push(transaction.clone());
        }
    }

    valid_transactions
}
```
The `is_valid_transaction` function in the `rustblock/src/validate_transactions.rs` file checks the transaction ,if it is pure P2PKH then call the function `validate_p2pkh` from the `rustblock/src/validation_scripts/p2pkh.rs` file and if it is P2WPKH then call the function `validate_p2wpkh` from the `rustblock/src/validation_scripts/p2wpkh.rs` file. for checking the transaction is valid or not. oherwise return false. ( I have not implemented the validation logic for other types of transactions , Because i got the enough score by validating only the P2PKH and P2WPKH transactions.)

```rust
fn is_valid_transaction(t: &Transaction) -> bool {
    // if the transaction is P2WPKH then
     if !p2wpkh_validate(t, i) {
                    return false;
                }
    //  else if the transaction is P2PKH then
    if !p2pkh_validate(t, i) {
                    return false;
                }
    // if the transaction is not P2PKH and P2WPKH then return false
    false

    // if non of the above condition is true then return true
    true
}
```
#### The validation logic for the P2PKH
The validation logic for the P2PKH transactions is implemented in the `rustblock/src/validation_scripts/p2pkh.rs` file. 
The  scriptpubkey_asm for the `P2PKH` is <br>
-> "OP_DUP OP_HASH160 OP_PUSHBYTES_20 ***< PubKeyHash >*** OP_EQUALVERIFY OP_CHECKSIG", <br>
The pub key and signature can be found in ScriptSig field of the input.<br>

So first we need to verify the ***pubKKeyHash*** , The **Hash** of the public key is calculated by hashing the public key using the SHA-256 and RIPEMD-160 algorithms. The sudocode for the validation of the P2PKH transaction is as follows:
```rust
 let pub_key = hex::decode(pub_key).unwrap();
    let pub_key_hash256 = Sha256::digest(pub_key);
    let pub_key_ripemd160 = Ripemd160::digest(&pub_key_hash256);
    let pub_key_ripemd160_hex = hex::encode(pub_key_ripemd160);
    // This hash should be equal to the pubKeyHash in the scriptPubKey_asm
    if pub_key_ripemd160_hex != pub_key_hash {
        return false;
    }
```
The next part in  validation of the P2PKH transaction is to verify the signature. The signature is verified by using the public key and meassage.
The meassage is the hash of the transaction data. 
The seriliazed transaction data is calculated by concatenating the version, inputs, outputs, locktime of the transaction and then hashing the concatenated data using the SHA-256 algorithm. 
The signature is verified using the ECDSE algorithm. The sudocode for the validation of the P2PKH transaction is as follows:
```rust
    let tx_data = serialize_transaction_data(t);
    let pub_key = PublicKey::from_slice(&pub_key).unwrap();
    let secp = Secp256k1::verification_only();
    let signature =  Signature::from_der_lax(&signature_bytes)
    let message = Message::from_digest_slice(&Sha256::digest(transaction_hash)).unwrap();
    secp.verify_ecdsa(&message, &signature, &pub_key).is_ok()
```
In This way the P2PKH transaction is validated.

#### The validation logic for the P2WPKH
The validation logic for the P2WPKH transactions is implemented in the `rustblock/src/validation_scripts/p2wpkh.rs` file.
The validation logic for the P2WPKH transactions is similar to the P2PKH transactions. 
The difference is that the scriptsig field is empty and the witness field contains the signature and public key. 
As here we are using the Segregated Witness (SegWit) format, the validation logic is slightly different. The way of seriliazing the transaction data is different.
    Double SHA256 of the serialization of:<br>
     1. nVersion of the transaction (4-byte little endian)<br>
     2. hashPrevouts (32-byte hash).<br>
     3. hashSequence (32-byte hash).<br>
     4. outpoint (32-byte hash + 4-byte little endian) .<br>
     5. scriptCode of the input (serialized as scripts inside CTxOuts).<br>
     6. value of the output spent by this input (8-byte little endian).<br>
     7. nSequence of the input (4-byte little endian).<br>
     8. hashOutputs (32-byte hash).<br>
     9. nLocktime of the transaction (4-byte little endian).<br>
     10. sighash type of the signature (4-byte little endian).<br>
    is used as message for the signature verification. <br>






### Creating the coinbase transaction
 The implementation of the coinbase transaction is in the `rustblock/src/coinbase_transaction.rs' file. The coinbase transaction is a special type of transaction that does not have any inputs and is used to collect the block reward. The coinbase transaction is created by the `create_coinbase_transaction` function in the `rustblock/src/coinbase_transaction.rs` file. The function creates a coinbase transaction with the following structure:   
  it contains 1 input and 2 outputs. The first output is the block reward and miner fee , second output is for the witness commitment. 
  the coinbase transaction is created as follows:
```rust
pub fn construct_coinbase_transaction(
    block_reward: u64,
    transaction_fees: u64,
    transactions: Vec<Transaction>,
) -> Transaction {
   // some other code
    let mut coinbase_inputs = Vec::new();
    let coinbase_input = TransactionInput {
        // There is no previous output for the coinbase transaction so the txid is set to all zeros
        txid: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        vout: 4294967295, // Coinbase transaction does not reference any previous output,any high value can be used
        prevout: PrevOut {
           // others fields that are not required for the coinbase transaction , i put empty string 
            value: 0,
        },
        // The coinbase transaction has a special scriptSig field that contains the block height and extra nonce,it is used to collect the block reward. just a random string is used for the scriptSig field.
        scriptsig: String::from(
            "03233708184d696e656420627920416e74506f6f6c373946205b8160a4256c0000946e0100",
        ),
       // the witness for the input is all zeros '0000000000000000000000000000000000000000000000000000000000000000'
        witness: wit,
       // very high value is used for the sequence field
        sequence: 0xFFFFFFFF,
    };
    coinbase_inputs.push(coinbase_input);

    let mut coinbase_outputs = Vec::new();
    let miner_output = TransactionOutput {
        scriptpubkey:String::from("41047eda6bd04fb27cab6e7c28c99b94977f073e912f25d1ff7165d9c95cd9bbe6da7e7ad7f2acb09e0ced91705f7616af53bee51a238b7dc527f2be0aa60469d140ac"), // Use the generated script 
         ... some other fields
        value: block_reward + transaction_fees
    };
    let miner_output2 = TransactionOutput {
        scriptpubkey: calculate_witness_commitment(transactions).to_string(),
        // others fields that are not required for the coinbase transaction , i put empty string
        value: 0,
    };
   .....
}
```
The the witness  commitment is calculated by the `calculate_witness_commitment` function in the `rustblock/src/coinbase_transaction.rs` file. The function calculates the witness commitment by hashing the Wtxids (merkle root )of the transactions in the block. The witness commitment is used to commit to the witness data of the block. The witness commitment is calculated as follows:
```rust
fn calculate_witness_commitment(transactions: Vec<Transaction>) -> String {
    let mut wtxids: Vec<String> = Vec::new();
    // witness for coinbase  transaction
    wtxids.push("0000000000000000000000000000000000000000000000000000000000000000".to_string());
    for t in transactions {
        let wtxid = wtxid_data(t);
        wtxids.push(wtxid);
    }
    let mut merkle_root = merkle_root(wtxids);

    // witness reserved value of coinbase transaction
    merkle_root.push_str("0000000000000000000000000000000000000000000000000000000000000000");

    let witness_commitment = double_sha256(merkle_root);
    // fix prefix for witness commitment "6a24aa21a9ed"
    let mut wit_new = String::from("6a24aa21a9ed".to_string());
    wit_new.push_str(&witness_commitment);
    wit_new
}
```
the 'wtxid_data' function in the `rustblock/src/coinbase_transaction.rs` file calculates the Wtxid of the transaction. The Wtxid is calculated by hashing the transaction data. The details of the Wtxid calculation can be found in the `rustblock/src/coinbase_transaction.rs` file.


### Assemble the Block
 Assemble the block by including the coinbase transaction and valid transactions in the block. The block is assembled by creating a block header and adding the coinbase transaction and valid transactions to the block.
the structure of the block is as follows:
```rust
pub struct Block {
    pub version: String,
    pub prev_block_hash: String, 
    pub merkle_root: String,
    pub timestamp: String,
    pub bits: String,
    pub nonce: u32,
    pub transactions: Vec<Transaction>,
}
```
The sudo code for the block assembly is as follows: `rustblock/src/assemble_block.rs`
```rust
pub fn assemble_block(transactions: Vec<Transaction>) -> Block {
    // Calculate the merkle root of the transactions
    let txids = calculate_txids(&transactions);
    let merkle_root = merkle_root(txids.clone());

    // assemble the block
    let block = Block {
        version: "04000000".to_string(),
        prev_block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
            .to_string(),
        merkle_root: merkle_root,
        // unix timestamp
        timestamp: convert_to_4bytes( SystemTime::now().duration_since(UNIX_EPOCH).unwrap() .as_secs() as u32,),
        bits: reverse_bytes("1f00ffff".to_string()),
        nonce: 0,
        transactions: transactions,
    };

    block
}
```
The `calculate_txids` function calcultes the txids of the transactions. THe implementation of the function can be found in the `rustblock/src/utiles.rs` file.

The `merkle_root` function calculates the merkle root of the all txids. The implementation of the function can be found in the `rustblock/src/utiles.rs` file.

### Mining the Block
The mining of the block is done by finding a hash that is less than the difficulty target. The difficulty target is a value that determines the difficulty of mining a block. The block header is created by combining the version, previous block hash, merkle root, timestamp, bits, and nonce fields of the block. The hash of the block header is calculated using the SHA-256 algorithm. The nonce value is adjusted until a valid hash is found that is less than the difficulty target. The implementation of the mining process can be found in the `rustblock/src/mine_block.rs` file.

The sudo code for the mining process is as follows:
```rust
pub fn mine_block(mut block: Block, target: String) -> Block {
   .
   .
   .
    let mut nonce = 0;
    loop {
        // Hash the block header
        let attempt = format!("{}{}", header_data, (convert_to_4bytes(nonce)));
        let result = reverse_bytes(double_sha256(attempt));

        // Break if the hash is below the target
        if BigUint::from_str_radix(&result, 16).unwrap()
            < BigUint::from_str_radix(&target, 16).unwrap()
        {
            break;
        }

        nonce += 1;
    }

    block.nonce = nonce;
    return block;
}
```

By following the above steps, the program is able to validate transactions, create a coinbase transaction, assemble the block, and mine the block. The block header and the serialized coinbase transaction are written to the `output.txt` file. The transaction IDs of the transactions mined in the block are also written to the `output.txt` file.








## Results and Performance 
The results of the solution are as follows: <span  style="color:red "> 98/100 </span>   score is achieved by validating the P2PKH and P2WPKH transactions. 

The program took around  <span  style="color:red "> 55  </span>  seconds to validate the transactions and mine them into a block. The program is able to validate around 8131 transactions and mine them into a block. The block contains the coinbase transaction and 3102 transactions. The block header and the serialized coinbase transaction are written to the `output.txt` file. The transaction IDs of the transactions mined in the block are also written to the `output.txt` file.


## Conclusion
 By solveing this problem, I have gained insights into the process of validating transactions and mining them into a block. I have learned about the key concepts of creating a valid block, including transaction validation, coinbase transactions, and mining the block. I have also gained experience in working with the `serde_json` crate to parse the json file and the `sha2` crate to calculate the hash of the block header.
The potential areas for future improvement or research include implementing the validation logic for other types of transactions, optimizing the code for better performance. I would also like to explore the use of parallel processing to speed up the validation and mining process.

#### References:
- [Bitcoin Developer Guide](https://developer.bitcoin.org/)
- [Learn me bicoin](https://learnmeabitcoin.com/)
- [ Articles on Bitcoin by otto ](https://medium.com/@ottosch)