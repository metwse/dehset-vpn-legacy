//! Pre-authentication messages.
//!
//! The current protocol version implements a fairly simple and straightforward
//! symmetric encryption mechanism. Nodes and servers are assumed to already
//! share knowledge of the symmetric encryption key (e.g., AES).

use crate::token::SignedToken;
use serde::{Serialize, Deserialize};

/// Content type of handshake payloads.
pub enum HandshakeContentType {
    /// Error indicating payload.
    HandshakeError = 0,
    /// Initial client hello with version and encryption algorithm.
    ClientHello = 1,
    /// Server response indicating protocol compatibility.
    ServerHello = 2,
    /// Token-based authentication payload.
    Authenticate = 3,
}

/// Error types that may occur during the handshake process.
#[derive(Serialize, Deserialize)]
pub enum HandshakeErrorType {
    /// The protocol version provided by the client is not supported by the
    /// server.
    IncompatibleVersion,
    /// The encryption algorithm selected by the client is not compatible with
    /// the server.
    IncompatibleAlgorithm,
    /// Token-based authentication failed.
    AuthenticationFailed,
}

/// Represents an error that occurred during the handshake phase.
#[derive(Serialize, Deserialize)]
pub struct HandshakeError {
    pub error_type: HandshakeErrorType,
    pub details: Option<String>,
}

/// Initial payload sent by the client, indicating the protocol version
/// and preferred encryption algorithm.
#[derive(Serialize, Deserialize)]
pub struct ClientHello {
    pub version: u16,
    pub encryption_algorithm: u8,
}

/// Server response indicating whether the client's configuration
/// is accepted.
#[derive(Serialize, Deserialize)]
pub struct ServerHello {
    pub accept: bool,
    pub random: [u8; 32],
}

/// Encrypted token used for authentication.
#[derive(Serialize, Deserialize)]
pub struct Authenticate {
    pub token: SignedToken,
    pub random: [u8; 32],
}
