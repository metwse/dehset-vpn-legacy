use super::{SignatureAlgorithm as TSignatureAlgorithm, Signer as TSigner, Verifier as TVerifier};
use crate::CryptoError;
use openssl::{
    hash::MessageDigest,
    memcmp,
    pkey::{PKey, Private},
    sign::Signer,
};
use proto_core::SignatureAlgorithm;

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
}

impl TSignatureAlgorithm for Hs256 {
    fn algorithm() -> SignatureAlgorithm {
        SignatureAlgorithm::HmacSha256
    }
}

impl TSigner for Hs256 {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key)?;
        signer.update(data)?;

        Ok(signer.sign_to_vec()?)
    }
}

impl TVerifier for Hs256 {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, CryptoError> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key)?;
        signer.update(data)?;

        Ok(memcmp::eq(&signer.sign_to_vec()?, signature))
    }
}
