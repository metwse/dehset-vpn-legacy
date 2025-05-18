//! Payloads of the underlying application layer.
//!
//! Represent tunneled data flowing through established connections, including
//! initial connection requests, raw application data, and connection
//! termination messages.

/// Type of application-layer content encapsulated in the VPN tunnel.
pub enum ApplicationDataContentType {
    /// Request to initiate a new connection to a given port.
    ConnectPort = 0,
    /// Arbitrary application data transmitted over an open connection.
    ApplicationData = 1,
    /// Signal to close a specific connection.
    CloseConnection = 2,
}

/// Message sent to request a connection to a specific port on the remote host.
pub struct ConnectPort {
    /// The target port number to connect to.
    pub port: u16,
    /// A connection ID that is unique per node and used to identify
    /// subsequent data transmissions related to this connection.
    pub connection_id: u64,
}

/// Carries raw application data through the tunnel.
pub struct ApplicationData {
    /// The binary payload to transmit.
    pub data: Vec<u8>,
    /// ID of the connection this data belongs to.
    pub connection_id: u64,
}

/// Message to close an existing connection identified by its ID.
pub struct CloseConnection {
    /// ID of the connection to be terminated.
    pub connection_id: u64,
}
