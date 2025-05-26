use bincode::error::{DecodeError, EncodeError};
use crypto::CryptoError;
use std::io::Error as IoError;

/// Handshake error types.
#[derive(Debug)]
pub enum Error {
    /// Typically indicate a fundamental problem with cryptographic setup.
    Crypto(CryptoError),
    /// I/O-related error (e.g., binding socket).
    Io(IoError),
    /// Handshake error.
    Handshake(&'static str),
    /// Decoding error.
    Decode(DecodeError),
    /// Encoding related errors.
    Encode(EncodeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crypto(crypto_error) => write!(f, "crypto: {crypto_error}"),
            Self::Io(io_error) => write!(f, "io: {io_error}"),
            Self::Handshake(handshake_error) => write!(f, "io: {handshake_error}"),
            Self::Decode(decode_error) => write!(f, "decode: {decode_error}"),
            Self::Encode(encode_error) => write!(f, "encode: {encode_error}"),
        }
    }
}

impl std::error::Error for Error {}

proto_core::error_impl_from!(Error; Crypto, Io, Decode, Encode);
