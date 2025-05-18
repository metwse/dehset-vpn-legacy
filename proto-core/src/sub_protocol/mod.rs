//! Message type definitions used across the protocol layers.
//!
//! High-level message types exchanged between peers, including handshake,
//! commands, application data, and alerts are defined in this module.

pub mod alert;
pub mod application_data;
pub mod cmd;
pub mod handshake;

pub use application_data::ApplicationDataContentType;
pub use cmd::{CmdContentType, CmdResponseContentType};
pub use handshake::{HandshakeContentType, HandshakeErrorType};

/// Type of content carried in the protocol payload.
pub enum ContentType {
    /// Pre-authentication handshake payloads (e.g. key exchange).
    Handshake = 0,
    /// Command messages used for control-plane operations in the VPN protocol.
    Cmd = 1,
    /// Underlying user application data and connectoin requests tunneled
    /// through the VPN.
    ApplicationData = 2,
    /// Alert messages used for signaling errors or warnings in the protocol.
    Alert = 3,
}
