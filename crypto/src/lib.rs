//! Protocol-level cryptographic functions.

#[cfg(test)]
use testutil::*;

mod error;
pub mod sign;
pub mod symm;
pub mod tls;

pub use error::CryptoError;
