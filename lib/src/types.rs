
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::crypto::{PublicKey, Signature};
use crate::error::{BtcError, Result};
use crate::sha256::Hash;
use crate::utils::MerkleRoot;
use crate::U256;
use uuid::Uuid;

// ==============================
// Transactions
// ==============================

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
    ) -> Self {
        Self { inputs, outputs }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

// ==============================
// Block
// ==============================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    pub target: U256,
}

impl BlockHeader {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self { header, transactions }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }

    pub fn verify_transactions(
        &self,
        block_height: u64,
        utxos: &HashMap<Hash, TransactionOutput>,
    ) -> Result<()> {
        if self.transactions.is_empty() {
            return Err(BtcError::InvalidBlock);
        }

        // coinbase must be first
        for tx in &self.transactions {
            tx.verify(block_height, utxos)?;
        }

        Ok(())
    }
}

// ==============================
// Blockchain
// ==============================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    utxos: HashMap<Hash, TransactionOutput>,
    target: U256,
    blocks: Vec<Block>,
    #[serde(default, skip_serializing)]
    mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
            blocks: vec![],
            target: crate::MIN_TARGET,
            mempool: vec![],
        }
    }

    pub fn block_height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn mempool(&self) -> &[Transaction] {
        &self.mempool
    }

    pub fn utxos(&self) -> &HashMap<Hash, TransactionOutput> {
        &self.utxos
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    // ==========================
    // Add block
    // ==========================
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero() {
                return Err(BtcError::InvalidBlock);
            }
        } else {
            let last_block = self.blocks.last().unwrap();

            if block.header.prev_block_hash != last_block.hash() {
                return Err(BtcError::InvalidBlock);
            }

            if !block.header.hash().matches_target(block.header.target) {
                return Err(BtcError::InvalidBlock);
            }

            let calculated_merkle_root =
                MerkleRoot::calculate(&block.transactions);

            if calculated_merkle_root != block.header.merkle_root {
                return Err(BtcError::InvalidMerkleRoot);
            }

            if block.header.timestamp <= last_block.header.timestamp {
                return Err(BtcError::InvalidBlock);
            }

            block.verify_transactions(self.block_height(), &self.utxos)?;
        }

        // remove confirmed txs from mempool
        let block_tx_hashes: HashSet<_> = block
            .transactions
            .iter()
            .map(|tx| tx.hash())
            .collect();

        self.mempool
            .retain(|tx| !block_tx_hashes.contains(&tx.hash()));

        self.blocks.push(block);
        Ok(())
    }

    // ==========================
    // UTXO rebuild
    // ==========================
    pub fn rebuild_utxos(&mut self) {
        self.utxos.clear();

        for block in &self.blocks {
            for tx in &block.transactions {
                for input in &tx.inputs {
                    self.utxos
                        .remove(&input.prev_transaction_output_hash);
                }

                for output in &tx.outputs {
                    self.utxos.insert(output.hash(), output.clone());
                }
            }
        }
    }
}

// ==============================
// Transaction verification stub
// ==============================

impl Transaction {
    pub fn verify(
        &self,
        _block_height: u64,
        _utxos: &HashMap<Hash, TransactionOutput>,
    ) -> Result<()> {
        // we will implement full validation later
        Ok(())
    }
}
```

