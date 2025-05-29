//! Server-to-client alert messages.
//!
//! Defines critical connection-level alerts that the server may send to a
//! client, usually resulting in immediate disconnection or state change.
//! These alerts are intended to notify the client of serious issues that
//! require attention.

use serde::{Deserialize, Serialize};

/// Closure alerts sent from the server to clients.
#[derive(Debug, Serialize, Deserialize)]
pub enum Alert {
    /// The client's token has been invalidated.
    TokenRevoked,
    /// A duplicate connection attempt has been detected.
    AlreadyConnected,
    /// The same token was used from a different device.
    LoggedInFromAnotherComputer,
}
