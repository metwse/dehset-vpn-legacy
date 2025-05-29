//! Server-to-client event notifications.
//!
//! Event module defines the structures used for asynchronous, server-initiated
//! messages sent to clients. These events inform clients of connection-related
//! changes within the VPN network, including:
//!
//! - Notifications about other clients connecting or disconnecting
//! - Initial client lists upon successful authentication

use serde::{Deserialize, Serialize};

/// Represents a connected client.
#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub tags: Vec<String>,
    pub token_id: u64,
    /// The timestamp when the server accepted the client connection.
    pub timestamp: u64,
}

/// Events sent from the server to clients.
#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    /// Notifies the client about a newly connected client.
    ClientConnected(Client),

    /// Notifies the client that another client has disconnected.
    ClientDisconnected {
        token_id: u64,
        /// The timestamp when the server closed the client connection.
        timestamp: u64,
    },

    /// Initial payload sent by the server to inform the client about
    /// currently active connections.
    ListClients(Vec<Client>),
}
