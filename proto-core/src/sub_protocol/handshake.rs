//! Pre-authentication messages.
//!
//! The current protocol version implements a fairly simple and straightforward
//! symmetric encryption mechanism. Nodes and servers are assumed to already
//! share knowledge of the symmetric encryption key (e.g., AES).

use crate::token::SignedToken;

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
pub struct HandshakeError {
    pub error_type: HandshakeErrorType,
    pub details: Option<String>,
}

/// Initial payload sent by the client, indicating the protocol version
/// and preferred encryption algorithm.
pub struct ClientHello {
    pub version: u16,
    pub encryption_algorithm: u8,
}

/// Server response indicating whether the client's configuration
/// is accepted.
pub struct ServerHello {
    pub accept: bool,
}

/// Encrypted token used for authentication.
pub struct Authenticate {
    pub token: SignedToken,
}
