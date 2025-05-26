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
#[derive(PartialEq, Debug)]
pub enum ContentType {
    /// Pre-authentication handshake payloads (e.g. key exchange).
    Handshake = 0,
    /// Command messages used for control-plane operations in the VPN protocol.
    Cmd = 1,
    /// Underlying user application data and connection requests tunneled
    /// through the VPN.
    ApplicationData = 2,
    /// Alert messages used for signaling errors or warnings in the protocol.
    Alert = 3,
}

impl std::convert::TryFrom<u8> for ContentType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Handshake),
            1 => Ok(Self::Cmd),
            2 => Ok(Self::ApplicationData),
            3 => Ok(Self::Alert),
            _ => Err(()),
        }
    }
}
