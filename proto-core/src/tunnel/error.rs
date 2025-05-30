use crate::error_impl_from;
use std::io::Error as IoError;

/// Tunnel error types.
#[derive(Debug)]
pub enum TunnelError {
    Io(IoError),
    Crypto,
    Disconnected,
    PayloadTooLarge,
}

impl std::fmt::Display for TunnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io_error) => write!(f, "io: {io_error}"),
            Self::Crypto => write!(f, "a crypto error is occured"),
            Self::Disconnected => write!(f, "disconnected"),
            Self::PayloadTooLarge => write!(f, "payload is too large"),
        }
    }
}

error_impl_from!(TunnelError; Io);

impl std::error::Error for TunnelError {}
