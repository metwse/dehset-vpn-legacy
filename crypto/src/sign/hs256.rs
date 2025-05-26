use crate::CryptoError;
use openssl::{
    hash::MessageDigest,
    memcmp,
    pkey::{PKey, Private},
    sign::Signer,
};

/// HMAC with SHA 256 symmetric keyed signature algorithm.
#[derive(Debug)]
pub struct Hs256 {
    key: PKey<Private>,
}

impl Hs256 {
    pub fn try_new(key: &[u8]) -> Result<Self, CryptoError> {
        Ok(Self {
            key: PKey::hmac(key)?,
        })
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key)?;
        signer.update(data)?;

        Ok(signer.sign_to_vec()?)
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, CryptoError> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key)?;
        signer.update(data)?;

        Ok(memcmp::eq(&signer.sign_to_vec()?, signature))
    }
}
