//! VPN client construction utilities.
//!
//! Configure and initialize a VPN client using [`ClientBuilder`].

mod client;
mod error;

pub use client::Client;
pub use error::Error;

use proto_core::SignedToken;
use std::net::SocketAddr;

/// Configuration structure for building and launching a VPN server instance.
///
/// Information used to initialize a client.
pub struct ClientBuilder {
    /// Address of the server.
    pub addr: SocketAddr,

    /// Symmetric encryption key used for encryption.
    pub encryption_key: Vec<u8>,

    /// ID token.
    pub token: SignedToken,
}
