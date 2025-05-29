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
