use std::fmt;

use crate::U256;

use serde::{Deserialize, Serialize};
use sha256::digest;

// ==========================================
// Hash Type
// ==========================================

#[derive(
    Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash,
)]
pub struct Hash(pub U256);

// ==========================================
// Core Hashing Logic
// ==========================================

impl Hash {
    /// Hash any serializable data using SHA-256
    pub fn hash<T: Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];

        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!(
                "Failed to serialize data: {:?}. This should not happen",
                e
            );
        }

        // sha256 crate returns hex string
        let hash_hex = digest(&serialized);
        let hash_bytes = hex::decode(hash_hex)
            .expect("sha256 returned invalid hex");

        let hash_array: [u8; 32] = hash_bytes
            .as_slice()
            .try_into()
            .expect("sha256 returned wrong length");

        Hash(U256::from(hash_array))
    }
}

// ==========================================
// Utility Methods
// ==========================================

impl Hash {
    /// Check if a hash satisfies the mining target
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    /// Return a zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }

    /// Convert hash to raw bytes (little endian)
    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        self.0.to_little_endian(&mut bytes);
        bytes
    }
}

// ==========================================
// Display Formatting
// ==========================================

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
```
