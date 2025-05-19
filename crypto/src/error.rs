use openssl::error::{Error as OpenSslError, ErrorStack as OpenSslStackError};

/// Crypto error types.
#[derive(Debug)]
pub enum CryptoError {
    OpenSslStack(OpenSslStackError),
    OpenSsl(OpenSslError),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenSslStack(openssl_error_stack) => {
                write!(f, "openssl_error_stack: {openssl_error_stack}")
            }
            Self::OpenSsl(openssl_error) => write!(f, "openssl_error: {openssl_error}"),
        }
    }
}

impl std::error::Error for CryptoError {}

proto_core::error_impl_from!(CryptoError; OpenSsl, OpenSslStack);
