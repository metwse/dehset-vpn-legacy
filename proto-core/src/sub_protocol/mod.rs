//! Message type definitions used across the protocol layers.
//!
//! High-level message types exchanged between peers, including handshake,
//! commands, application data, and alerts are defined in this module.

pub mod alert;
pub mod application_data;
pub mod cmd;
pub mod cmd_response;
pub mod event;
pub mod handshake;

/// Identifies the type of protocol message contained in a payload.
/// Used to route and deserialize messages correctly based on their category.
pub enum ContentType {
    Alert = 0,
    ApplicationData = 1,
    Cmd = 2,
    CmdResponse = 3,
    Event = 4,
}
