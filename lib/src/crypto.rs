use ecdsa::{
    signature::Signer,
    Signature as ECDSASignature,
    SigningKey,
    VerifyingKey
    
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(pub ECOSASignature<Secp256k1>);

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
