use bincode::error::{DecodeError, EncodeError};
use crypto::CryptoError;
use proto_core::handshake::HandshakeAlert as HandshakeError;
use std::io::Error as IoError;

/// Client error types.
#[derive(Debug)]
pub enum Error {
    Crypto(CryptoError),
    Io(IoError),
    Encode(EncodeError),
    Decode(DecodeError),
    Handshake(HandshakeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crypto(crypto_error) => write!(f, "crypto: {crypto_error}"),
            Self::Io(io_error) => write!(f, "io: {io_error}"),
            Self::Encode(encode_error) => write!(f, "encode: {encode_error}"),
            Self::Decode(decode_error) => write!(f, "encode: {decode_error}"),
            Self::Handshake(handshake_alert) => write!(f, "handshake: {handshake_alert:?}"),
        }
    }
}

impl std::error::Error for Error {}

proto_core::error_impl_from!(Error; Crypto, Io, Encode, Decode, Handshake);
