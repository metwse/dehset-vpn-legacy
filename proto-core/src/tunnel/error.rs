use std::io::Error as IoError;
use crate::error_impl_from;

pub enum TunnelError {
    Io(IoError),
    Crypto,
    Disconnected,
}

impl std::fmt::Display for TunnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io_error) => write!(f, "io: {io_error}"),
            Self::Crypto => write!(f, "a crypto error is occured"),
            Self::Disconnected => write!(f, "disconnected"),
        }
    }
}

error_impl_from!(TunnelError; Io);
