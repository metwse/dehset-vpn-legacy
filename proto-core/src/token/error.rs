use bincode::error::EncodeError;

/// Token operations error types.
#[derive(Debug)]
pub enum TokenError {
    Encode(EncodeError),
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(encode_error) => write!(f, "encode: {encode_error}"),
        }
    }
}

impl std::error::Error for TokenError {}

crate::error_impl_from!(TokenError; Encode);
