//! Encryption and decryption primitives used during handshake, authentication,
//! and application data phases of the protocol.

mod aes_128_cbc;

pub use aes_128_cbc::*;
