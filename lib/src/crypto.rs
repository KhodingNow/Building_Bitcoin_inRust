
use crate::sha256::Hash;

use serde::{Deserialize, Serialize};

use ecdsa::{
    signature::{Signer, Verifier},
    Signature as EcdsaSignature,
    SigningKey,
    VerifyingKey,
};

use k256::Secp256k1;

// ==========================================
// Key Types
// ==========================================

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(pub VerifyingKey<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(pub SigningKey<Secp256k1>);

impl PrivateKey {
    pub fn new() -> Self {
        Self(SigningKey::random(&mut rand::thread_rng()))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}

// ==========================================
// Signature Type
// ==========================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(pub EcdsaSignature<Secp256k1>);

// ==========================================
// Signing & Verification
// ==========================================

impl Signature {
    /// Sign a hash using a private key
    pub fn sign(hash: &Hash, private_key: &PrivateKey) -> Self {
        let signature = private_key.0.sign(hash.as_bytes());
        Self(signature)
    }

    /// Verify a signature against a public key
    pub fn verify(
        &self,
        hash: &Hash,
        public_key: &PublicKey,
    ) -> bool {
        public_key
            .0
            .verify(hash.as_bytes(), &self.0)
            .is_ok()
    }
}
```
