//! Handshake TLS providers.

use crate::{CryptoError, symm::Aes128CbcSha256};
use proto_core::TlsProvider;
use std::sync::{Arc, Mutex};

/// Encrption layer implementing symmetric encrption.
pub struct SymmTls {
    encrpyt_iv: Mutex<[u8; 32]>,
    decrypt_iv: Mutex<[u8; 32]>,
    encrpter: Arc<Aes128CbcSha256>,
}

impl SymmTls {
    /// Creates a new [`SymmTls`].
    pub fn new(
        (server_iv, client_iv): ([u8; 32], [u8; 32]),
        encrpter: Arc<Aes128CbcSha256>,
    ) -> SymmTls {
        let mut hasher = openssl::sha::Sha256::new();
        hasher.update(&server_iv);
        hasher.update(&client_iv);
        let iv = hasher.finish();

        SymmTls {
            decrypt_iv: Mutex::new(iv),
            encrpyt_iv: Mutex::new(iv),
            encrpter,
        }
    }
}

macro_rules! increment_iv {
    ($iv:expr) => {{
        let mut i = 0;
        loop {
            if $iv[i] == 255 {
                $iv[i] = 0;
                i += 1;
                if $iv.len() == i {
                    i = 0;
                }
                continue;
            }
            $iv[i] += 1;
            break;
        }
    }};
}

impl TlsProvider for SymmTls {
    type Error = CryptoError;

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let mut iv = self.encrpyt_iv.lock().unwrap();
        increment_iv!(iv);

        let iv = iv.clone();

        let mut shasum = Vec::from(self.encrpter.shasum(data));
        shasum.append(&mut self.encrpter.encrypt(Some(&iv), data)?);

        Ok(shasum)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let mut iv = self.decrypt_iv.lock().unwrap();
        increment_iv!(iv);

        let iv = iv.clone();

        let payload = self.encrpter.decrypt(Some(&iv), &ciphertext[32..])?;
        let shasum = self.encrpter.shasum(&payload);

        if shasum == &ciphertext[0..32] {
            Ok(payload)
        } else {
            Err(CryptoError::InvalidShasum)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SymmTls;
    use crate::{CryptoError, symm::Aes128CbcSha256};
    use proto_core::{TlsProvider, random_bytes};
    use std::sync::Arc;
    use testutil::DynResult;

    #[test]
    fn symm_tls_fuzz() -> DynResult<()> {
        for _ in 0..128 {
            let iv = random_bytes!(32);
            let key = Vec::from(random_bytes!(16));
            let server_tls = SymmTls::new((iv, iv), Arc::new(Aes128CbcSha256::new(key.clone())));
            let client_tls = SymmTls::new((iv, iv), Arc::new(Aes128CbcSha256::new(key)));

            for _ in 0..16 {
                let payload = Vec::from(random_bytes!(16));

                assert_eq!(payload, client_tls.decrypt(&server_tls.encrypt(&payload)?)?);
                assert_eq!(payload, server_tls.decrypt(&client_tls.encrypt(&payload)?)?);
            }
        }

        Ok(())
    }

    #[test]
    fn symm_tls_invalid() -> DynResult<()> {
        let iv = random_bytes!(32);
        let key = Vec::from(random_bytes!(16));
        let symm_tls = SymmTls::new((iv, iv), Arc::new(Aes128CbcSha256::new(key)));

        let payload1 = random_bytes!(32);
        let payload2 = random_bytes!(32);

        let _ciphertext1 = symm_tls.encrypt(&payload1)?;
        let ciphertext2 = symm_tls.encrypt(&payload2)?;

        if let Err(CryptoError::InvalidShasum) = symm_tls.decrypt(&ciphertext2) {
            Ok(())
        } else {
            panic!("Expected CryptoError::InvalidShasum");
        }
    }
}
