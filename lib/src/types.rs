use crate::U256;
use uuid::Uuid;

pub struct BlockHeader {

    /// Timestamp of the block
    pub timestamp: u64,
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


pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {

    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    
 }

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
            transactions: transaction,
            
        }
    } 
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: u64,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8, 32],
        target: U256,
        
    ) -> Self {
        BlockHeader {
            timestampt;
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pun fn hash(&self) -> ! {
        unimplented! ()
    }
}

pub struct Transaction{
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8, 32],
    pub signature: [u8; 64],
}

pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33],
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
        pub fn hash(&self) -> ! {
            unimplemented!()
        }
}
