use crate::CryptoError;
use openssl::{
    sha,
    symm::{Cipher, decrypt, encrypt},
};

/// AES 128 with CBC encryption + SHA256 algorithm.
#[derive(Debug, Clone)]
pub struct Aes128CbcSha256 {
    key: Vec<u8>,
}

impl Aes128CbcSha256 {
    pub fn new(key: Vec<u8>) -> Self {
        Aes128CbcSha256 { key }
    }

    pub fn encrypt(&self, iv: Option<&[u8]>, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Ok(encrypt(Cipher::aes_128_cbc(), &self.key, iv, payload)?)
    }

    pub fn decrypt(&self, iv: Option<&[u8]>, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Ok(decrypt(Cipher::aes_128_cbc(), &self.key, iv, payload)?)
    }

    pub fn shasum(&self, payload: &[u8]) -> [u8; 32] {
        let mut hasher = sha::Sha256::new();
        hasher.update(payload);
        hasher.finish()
    }
}
