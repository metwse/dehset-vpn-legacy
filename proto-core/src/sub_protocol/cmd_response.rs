//! Server response payloads for command requests.

use serde::{Deserialize, Serialize};

/// Response types for the `Authenticate` command payload.
#[derive(Debug, Serialize, Deserialize)]
pub enum Authenticate {
    Success,
    InvalidToken,
    AlreadyConnected,
    /// TODO: Implement token revocation support.
    TokenRevoked,
}

/// Command-level response payloads.
#[derive(Debug, Serialize, Deserialize)]
pub enum CmdResponsePayload {
    Authenticate(Authenticate),
}

/// Payload structure wrapping a command response with an identifier.
#[derive(Debug, Serialize, Deserialize)]
pub struct CmdResponse {
    pub response_id: u64,
    pub payload: CmdResponsePayload,
}
