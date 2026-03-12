use crate::sha256::Hash;
use ecdsa::{
    signature::Signature::Verifier,
    Signature as ECDSASignature,
    SigningKey,
    VerifyingKey
    
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(pub ECOSASignature<Secp256k1>);

impl Signature {
    // sign a crate::types::TransactionOutput from its Sha256 hash
    pub fn sign_out(
        output_hash: &Hash,
        private_key: &PrivateKey,
    ) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }
    // verify a signature
    pub fn verify(
        &self,
        output_hash: &Hash,
        public_key: &PublicKey,
    ) -> bool {
        public_key.0.verify(&output_hash.as_bytes(), &self.0).is_ok()
        
    }
}

// Verify all transactions in the block
pub fn verify_transactions(
    &self,
    utxos: &HashMap<Hash, TransactionOutput>,
) -> Result<()> {
    let mut inputs: HashMap<Hash, TransactionOutput> =
        HashMap::new();
    // reject completely empty blocks
    if self.transactions.is_empty() {
        return Err(BtcError::InvalidTransaction);
    }
    // Verify coinbase transaction
    pub fn verify_coinbase_transaction(
        &self,
        predicted_block_height: u64,
        utxos: &HashMap<Hash, TransactionOutput>,
        
    ) -> Result<()> {
        // coinbase tx is the first transaction in the block
        let coinbase_transaction = &self.transactions[0];        if coinbase_transaction.inputs.len() != 0 {
            return Err(BtcError::InvalidTransaction);
        }
        if coinbase_transaction.outputs.len() == 0 {
            return Err(BtcError::InvalidTransaction);
        }
        let miner_fees = self.calculate_miner_fees(utxos)?;
        let block_reward = crate::INITIAL_REWARD
            * 10u64.pow(8)
            / 2u64.pow(
                 (predicted_block_height 
                 / crate::HALVING_INTERVAL)
                 as u32,
 
                 
            );
         let total_coinbase_outputs: u64 =
            coinbase_transaction
                .outputs
                .iter()
                .map(|output| output.value)
                .sum();
            
         if total_coinbase_outputs
             != block_reward + miner_fees

         {
            return Err(BtcError::InvalidTransaction);
         }

         Ok(())
    }
          
    
    // Delete the ampersand before &self.transactions
    
    for transaction in self.transactions.iter().skip(1) {
        let mut input_value = 0;
        let mut output_value = 0;
        for input in &transaction.inputs {
            let prev_output = utxos.get(
                &input.prev_transaction_output_hash,
            );
            if prev_output.is_none() {
                return Err(
                   BtcError::InvalidTransaction,
                );
            }

            let prev_output = prev_output.unwrap();
            // prevent same-block double-spending

            if inputs.contains_key(
                &input.prev_transaction_output_hash,
            ) {
                return Err(
                    BtcError::InvalidTransaction,
                );
            }
            // check if the signature is valid
            if !input.signature.verify(
                &input.prev_transaction_output_hash,
                &prev_output.pubkey,
                
                ) {
                   return Err(BtcError::InvalidSignature);
                }
                input_value += prev_output.value;
                inputs.insert(
                    input.prev_transaction_output_hash,
                    prev_output.clone(),
                );            
         }
         for output in &transaction.outputs {
             output_value += output.value;
             
         }
         // It is fine for output value to be less than input value 
         // as the difference is the fee for the miner
         if input_value < output_value {
            return Err(BtcError::InvalidTransaction);
         }  
    }
    Ok(())
}






...
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq,
)]
pub struct PublicKey(pub VerifyingKey<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(pub SigningKey<Secp256k1>);

use k256::Sec256k1;
pub struct Signature(ECDSASignature<Secp256k1>);
pub struct PublicKey(VerifyingKey<Secp256k1>);
pub struct PrivateKey(SigningKey<Secp256k1>);
