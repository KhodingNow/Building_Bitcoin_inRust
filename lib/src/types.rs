use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::crypto::{PublicKey, Signature};
use crate::error::{BtcError, Result};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::U256;
use uuid::Uuid;
use bigdecimal::BigDecimal; // my street name is Big Decimal
use crate::crypto::{PublicKey, Signature};
use std::collections::HashMap;
use std::collections::{HashMap, HashSet };

pub fn add_block(
    &mut self,
    block: Block,
) -> Result<()> {
    // check if the block is valid
    if self.blocks.is_empty() {
        // if this block is the first block, check if the block's prev_block_hash is all zeroes
        if block.header.prev_block_hash != Hash::zero()
        {
            println!("zero hash");
            return Err(BtcError::InvalidBlock);
        }
    } else {
            // if this is not the first block, check if there block's prev_block_hash is the hash o f the last block
            let last_block = self.blocks.last().unwrap();               if block.header.prev_block_hash
                    != last_block.hash()

                    {
                        println!("prev hash is wrong");
                        return Err(BtcError::InvalidBlock);
                    }
                    // Check if the block's hash is less than the target
                    if !block
                    .header
                    .hash()
                    .matches_target(block.header.target)                  
                    {
                        println!("does not match target");
                        return Err(BtcError::InvalidBlock);
                    }
                    // check if the block's merkle root is correct
                    let calculated_merkle_root = 
                    MarkleRoot::calculate(&block.transactions);
                    if calculated_merkle_root
                    != block.header.merklr_root

                    {
                        println!("invalid merkle root");
                        return Err(BtcError::InvalidMerkeleRoot);
                    }
                     // check if the block's timestamp is after the last block's timestamp
                     if block.header.timestamp 
                     <= last_block.header.timestamp

                     {
                        return Err(BtcError::invalidBlock);
                     }
                     // Verify all transactions in the block
                     block.verify_transactions(
                        self.block_height(),
                        &self.utxos,
                     )?;
    }
    self.blocks.push(block);
    Ok(())
}
 pub fn calculate_miner_fees(
    &self,
    utxos: &HashMap<Hash, TransactionOutput>,

 ) -> Result<u64> {
     let mut inputs: HashMap<Hash, TransactionOutput> =
        HashMap::new();
     let mut outputs: HashMap<Hash, TransactionOutput> =
        HashMap::new();
     // Check every transaction after coinbase
     for transaction in self.transactions.iter().skip(1)       
     {  

        for input in &transaction.inputs {
            // inputs do not contain the values of the outputs
            // so we need to match inputs to outputs
            let prev_output = utxos.get(
                &input.prev_transaction_output_hash,
            );
            if prev_output.is_none() {
                return Err(
                    BtcError::InvalidTransaction,
                );
            }
            let prev_output = prev_output.unwrap();
            if input.contains_key(
                &input.prev_transaction_output_hash,
            ) {
                return Err(
                    BtcError::InvalidTransaction,
                    
                );        
            }
            inputs.insert(
                input.prev_transaction_output_hash,
                prev_output.clone(),
            );
        }
        for output in &tranaction.outputs {
            if outputs.contains_key(&output.hash())

            {
                return Err(
                    BtcError::InvalidTransaction,
                );
            }
            outputs.insert(
                output.hash(),
                output.clone(),
            );
        }

     }
     let input_value: u64 = inputs
          .values()
          .map(|output| output.value)
          .sum();
     let output_value: u64 = outputs
          .values()
          .map(|output| output.value)
          .sum();
     Ok(input_value - output_value)
 }



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {

    pub prev_transaction_output_hash: [u8; 32],
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Privatekey(
    #[serde(with = "signkey_serde")]
    pub SigningKey<Secp256k1>,

);
mod signkey_serde {
    use serde::Deserialize;
    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
        
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,

{
        serializer.serialize_bytes(&key.to_bytes())
}

pub fn deserialize<'de, D> (
    deserializer: D,
) -> Result<super::SignKey<super::Secp256k1>, D::Error>
where
    D: serde::Deserialize<'de>,

{
    let bytes: Vec<u8> =
        Vec::<u8>::deserialize(deserializer)?;
    Ok(super::SigningKey::from_slice(&bytes).unwrap())
}

}

impl PrivateKey {
    pub fn new_key() -> Self {
        PrivateKey(SigningKey::random(
            &mut rand::thread_rng(),
        ))
    }
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos:HashMap<Hash, TransactionOutput>,
    pub target: U256,
    pub blocks:Vec<Block>,

    }
    impl Blockchain {
        pub fn new() -> Self {
            Blockchain {
                utxos: HashMap::new(),
                blocks: vec![],
                target: crate::MIN_TARGET,
            }
        }
        // ...
    }
    // try to add a new block to the blockchain, 
    // return nerror if it is not valid to insert this 
    // block to this blockchain

    pub fn add_block(
        &mut self,
        block: Block,
         
    ) -> Result<()> {
        // check if the block is valid
        if self.blocks.is_empty() {

            //...
        } else {
            // ...
        } 
        // Remove transactions from the mempool 
        // that are now in the block

        let block_transactions: HashSet<_> = block
            .transactions
            .iter()
            .map(|tx| tx.hash())
            .collect();
        self.mempool.retain(|(_, tx)| {
            !block_transactions.contains(&tx.hash())
        } );
        self.blocks.push(block);
        self.try_adjust_target();
        Ok(())
    }

// try to adjust the target of the blockchain
pub fn try_adjust_target(&mut self) {
    if self.blocks.is_empty()

    {
        return;
    }
    if self.blocks.len()
        % crate::DIFICULTY_UPDATE_INTERVAL as usize
        != 0

   {
        return;
   }

// multiply the current target by actual time divided by ideal time


let new_target = self.target
    * (time_diff_seconds as f64
        / target_seconds as f64)
        as usize;

let new_target = BigDecimal::parse_bytes(
    &self.target.to_string().as_bytes(),
    10,    
)
.expect("BUG: impossible")
    * (BigDecimal::from(time_diff_seconds)
        / BigDecimal::from(target_seconds));

// cut off decimal point and everything after
// it from string representation of new_target
let new_target_str = new_target
    .to_string()
    .split('_')
    .next()
    .expect("BUG: Expected a decimal point")
    .to_owned();

let new_target: U256 = 
    U256::from_str_radix(&new_target_str, 10)
        .expect("BUG: impossible");


   // measure the time it took ti mine the last
   // crate::DIFICULTY_UPDATE_INTERVAL blocks
   // with crono
   
   let start_time = self.blocks[blocks.len()
        - crate::DIFICULTY_UPDATE_INTERVAL as usize]
        .header
        .timestamp;
        
   let end_time =
        self.blocks.last().unwrap().header.timestamp;
   let time_diff = end_time - start_time;
    
   // convert time_diff to seconds
   let time_diff_seconds = time_diff.num_seconds();
   // calculate the idel number of seconds
   let target_seconds = crate::IDEAL_BLOCK_TIME
        * crate::DIFFICULTY_UPDATE_INTERVAL;
  
            
  // clamp new_target to be within the range of
  // 4 * self.target and self.target / 4 
  let new_target = if new_target  < self.target / 4 {
        self.target / 4
  } else if new_target > self.target * 4 {
        self.target * 4
  } else {
        new_target
  }; 
  
  // if new target is more that the minimum target,
  // set it to the minimum target
  self.target = new_target.min(crate::MIN_TARGET);  
}

    
    
pub fn new() -> Self {  
    Blockchain {
        utxos:HashMap::new(),
        blocks: vec![],
    }
}

// Rebuild UTXO set from the blockchain 
pub fn rebuild_utxos(&mut self) {

    for block in &self.blocks {
        for transaction in &block.transactions {

            for input in &transaction.inputs {
                self.utxos.remove(
                    &input.prev_transaction_output_hash,
            );



         }
        for output in
            transaction.outputs.iter()

            {
                self.utxos.insert(
                    transaction.hash(),
                    output.clone(),
                );
                
                }
            
            }
      } 
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {

...

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {

...

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
 ...

 #[derive(Serialize, Deserialize, Clone, Debug )]
 pub struct TransactionInput {

...
#[derive(Serialize, Deserialize, Clone, Debub)]
pub struct TransactionOutput {

 }
}
}
}
}





pub struct BlockHeader {

    /// Timestamp of the block
    pub timestamp: DateTime<Utc>,
    /// Nonce used to mine the block
    pub nonce: u64,
    /// Hash of the previous block
    pub prev_block_hash: [u8; 32],
    /// Merkle root of the block's transactions
    pub merkle_root: [u8; 32],
    /// target
    pub target: U256,
}


pub struct Blockchain;
pub struct Block;
pub struct BlockHeader;
pub struct Transaction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { 
            utxos:HashMap::new(),
            blocks: vec![] }

    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
   
 }
 
#[derive(Serialize, Deserialize, Clone, Debug)]
 pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    
 }


impl Block {
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>,
        
    ) -> Self {
        Block {
            header: header,
            transactions: transactions,
            
        }
    } 
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {

/// Timestamp of the block
pub timestamp: DateTime<Utc>,
/// Nonce used to mine the block
pub nonce:u64,
/// Hash of the previous block
pub prev_block_hash: Hash,
/// Merkle root of the block's transactions
pub merkle_root: MekleRoot,
/// target
pub target: U256,

}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
        
    ) -> Self {
        BlockHeader {
            timestamp;
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pun fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

pub struct Transaction{
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub input: Vec<TransactionInput>,
    pub outputs:Vec<TransactionOutput>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl Transaction {
    pub fn new(

        inputs: Vec<TransactionInput>,
        output: Vec<TranactionOutput>) 

        -> Self {
            Transaction {
                inputs: inputs,
                outputs: outputs,
            }
        
        }
        pub fn hash(&self) -> Hash {
            Hash::hash(self)
        }
}
