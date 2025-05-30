use super::{SignatureAlgorithm, Signer};
use crate::CryptoError;
use proto_core::token::{SignedToken, Token};

pub fn sign_token<S: Signer + SignatureAlgorithm>(
    token: Token,
    signer: &S,
) -> Result<SignedToken, CryptoError> {
    Ok(SignedToken {
        signature: signer.sign(&token.encode()?[..])?,
        token,
        signature_algorithm: S::algorithm(),
    })
}
