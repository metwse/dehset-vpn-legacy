//! Encryption and decryption primitives used during handshake, authentication,
//! and application data phases of the protocol.

mod aes_128_cbc;

#[cfg(test)]
mod tests;

pub use aes_128_cbc::*;
