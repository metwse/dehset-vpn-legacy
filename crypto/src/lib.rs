//! Protocol-level cryptographic functions.

#[cfg(test)]
mod testutil;

#[cfg(test)]
pub use testutil::*;

mod error;
pub mod sign;
pub mod symm;

pub use error::CryptoError;
