Apart from the code, you must also publish a `SOLUTION.md` file explaining your solution in the following format:
- **Design Approach:** Describe the approach you took to design your block construction program, explain all the key concepts of creating a valid block.
- **Implementation Details:** Provide pseudo code of your implementation, including sequence of logic, algorithms and variables used etc.
- **Results and Performance:** Present the results of your solution, and analyze the efficiency of your solution.
- **Conclusion:** Discuss any insights gained from solving the problem, and outline potential areas for future improvement or research. Include a list of references or resources consulted during the problem-solving process.


## Design Approach
The design approach for this problem is to create a block construction program that can validate transactions and mine them into a block. The key concepts of creating a valid block include:

- **Transaction Validation:** The program should be able to validate transactions based on the given set of transactions. This involves checking the transaction inputs, outputs, and signatures to ensure that they are valid. And to get the high score i sort all the transactions based on the weight;

- **coinbase Transaction:** The coinbase transaction is the first transaction in a block. It is a special type of transaction that does not have any inputs and is used to collect the block reward. 

- **Assemble Block:** The program should be able to assemble the block by including valid transactions in the block. The block should  includes the coinbase transaction.

- **Mining the Block:** The program should be able to mine the block by finding a hash that is less than the difficulty target. This involves calculating the hash of the block header and adjusting the nonce value until a valid hash is found.

- **Output:** The program should generate an output file named `output.txt` that follows a specific format. The output file should contain the block header, the serialized coinbase transaction, and the transaction IDs of the transactions mined in the block.

## Implementation Details

### The implementation of the block construction program involves the following steps:

1. Create a Struct `Transaction` to represent a transaction. The structure of the transaction is as follows:
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

// todo: add the pseudo code of the implementation












## Results and Performance 
The results of the solution are as follows: <span  style="color:red "> *98/100* </span>   score is achieved by validating the P2PKH and P2WPKH transactions. 

The program took around  <span  style="color:red "> *60 sec* </span>  seconds to validate the transactions and mine them into a block. The program is able to validate around 8131 transactions and mine them into a block. The block contains the coinbase transaction and 3102 transactions. The block header and the serialized coinbase transaction are written to the `output.txt` file. The transaction IDs of the transactions mined in the block are also written to the `output.txt` file.


## Conclusion
 By solveing this problem, I have gained insights into the process of validating transactions and mining them into a block. I have learned about the key concepts of creating a valid block, including transaction validation, coinbase transactions, and mining the block. I have also gained experience in working with the `serde_json` crate to parse the json file and the `sha2` crate to calculate the hash of the block header.
The potential areas for future improvement or research include implementing the validation logic for other types of transactions, optimizing the code for better performance. I would also like to explore the use of parallel processing to speed up the validation and mining process.

#### References:
- [Bitcoin Developer Guide](https://developer.bitcoin.org/)
- [Learn me bicoin](https://learnmeabitcoin.com/)
- [ Articles on Bitcoin by otto ](https://medium.com/@ottosch)