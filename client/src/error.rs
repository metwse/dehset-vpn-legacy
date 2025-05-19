use crypto::CryptoError;
use std::io::Error as IoError;

/// Non-recoverable client error types. See `server::Error`.
#[derive(Debug)]
pub enum Error {
    Crypto(CryptoError),
    Io(IoError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crypto(crypto_error) => write!(f, "crypto: {crypto_error}"),
            Self::Io(io_error) => write!(f, "io: {io_error}"),
        }
    }
}

impl std::error::Error for Error {}

proto_core::error_impl_from!(Error; Crypto, Io);
