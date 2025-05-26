//! Signing and verifying primitives used during handshake.

mod hs256;
mod token;

#[cfg(test)]
mod tests;

use crate::CryptoError;

pub use hs256::*;
pub use token::*;

/// Signing algorithm of the struct.
pub trait SignatureAlgorithm {
    fn algorithm() -> proto_core::SignatureAlgorithm;
}

/// Signer trait for signing tokens.
pub trait Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

/// Verifier trait for verifying tokens.
pub trait Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, CryptoError>;
}
