//! VPN-related administrative payloads.
//!
//! Messages used for control-plane operations such as signing and revoking
//! authentication tokens.

use crate::SignedToken;
use serde::{Deserialize, Serialize};

/// Initial authentication payloads that should be sent by all connecting
/// clients.
#[derive(Debug, Serialize, Deserialize)]
pub struct Authenticate {
    pub token: SignedToken,
}

/// Command-level response payloads.
#[derive(Debug, Serialize, Deserialize)]
pub enum Cmd {
    Authenticate(Authenticate),
}

/// Payload structure wrapping a command response with an identifier.
#[derive(Debug, Serialize, Deserialize)]
pub struct CmdPayload {
    pub response_id: u64,
    pub payload: Cmd,
}
