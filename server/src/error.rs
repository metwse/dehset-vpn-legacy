use crypto::CryptoError;
use std::io::Error as IoError;

/// Non-recoverable errors that may occur during server startup.
#[derive(Debug)]
pub enum Error {
    /// Indicates an issue with the cryptographic setup.
    Crypto(CryptoError),
    /// I/O-related error (e.g., failure to bind a socket).
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
