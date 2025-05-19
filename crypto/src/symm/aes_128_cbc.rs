use crate::CryptoError;
use openssl::symm::{Cipher, decrypt, encrypt};

pub struct Aes128Cbc {
    key: Vec<u8>
}

impl Aes128Cbc {
    pub fn new(key: Vec<u8>) -> Self {
        Aes128Cbc { key }
    }

    pub fn encrypt(self, iv: &[u8], payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Ok(encrypt(Cipher::aes_128_cbc(), &self.key, Some(iv), payload)?)
    }

    pub fn decrypt(self, iv: &[u8], payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Ok(decrypt(Cipher::aes_128_cbc(), &self.key, Some(iv), payload)?)
    }
}
