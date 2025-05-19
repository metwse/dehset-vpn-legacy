//! Encryption and decryption primitives used during handshake, authentication,
//! and application data phases of the protocol.

use crate::CryptoError;
use openssl::symm::{decrypt, encrypt, Cipher};

pub fn aes_128_cbc_encrypt(key: &[u8], iv: &[u8], payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Ok(encrypt(Cipher::aes_128_cbc(), key, Some(iv), payload)?)
}

pub fn aes_128_cbc_decrypt(key: &[u8], iv: &[u8], payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Ok(decrypt(Cipher::aes_128_cbc(), key, Some(iv), payload)?)
}
