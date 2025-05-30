use openssl::error::{Error as OpenSslError, ErrorStack as OpenSslStackError};
use proto_core::token::TokenError;

/// Crypto error types.
#[derive(Debug)]
pub enum CryptoError {
    OpenSslStack(OpenSslStackError),
    OpenSsl(OpenSslError),
    Token(TokenError),
    InvalidShasum,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenSslStack(openssl_error_stack) => {
                write!(f, "openssl error stack: {openssl_error_stack}")
            }
            Self::OpenSsl(openssl_error) => write!(f, "openssl error: {openssl_error}"),
            Self::Token(token_error) => write!(f, "token error: {token_error}"),
            Self::InvalidShasum => write!(f, "could not verify shasum"),
        }
    }
}

impl std::error::Error for CryptoError {}

proto_core::error_impl_from!(CryptoError; OpenSsl, OpenSslStack, Token);
