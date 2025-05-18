//! VPN-related administrative payloads.
//!
//! Messages used for control-plane operations such as signing and revoking
//! authentication tokens.

use crate::token::{SignedToken, Token};

/// Command message types sent to the server for administrative actions.
pub enum CmdContentType {
    SignToken = 0,
    RevokeToken = 1,
}

/// Response message types for command operations.
pub enum CmdResponseContentType {
    SignedToken = 0,
    RevocationSuccess = 1,
}

/// Request to sign a new token.
pub struct SignToken {
    pub token: Token,
}

/// Response containing the signed token.
pub struct SignTokenResponse {
    pub signed_token: Option<SignedToken>,
    pub success: bool,
}

/// Request to revoke an existing token by its ID.
pub struct RevokeToken {
    pub token_id: u64,
}

/// Acknowledgement of successful token revocation.
pub struct RevokeTokenResponse {
    pub success: bool,
}
