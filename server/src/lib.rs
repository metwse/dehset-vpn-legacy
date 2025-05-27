//! VPN server construction utilities.
//!
//! Configure and initialize a VPN server using [`ServerBuilder`].
//!
//! # Example
//! The following example demonstrates the most minimal server setup.
//! ```no_run
#![doc = include_str!("../examples/minimal-server.rs")]
//! ```

pub mod connection;
mod error;
mod server;

pub use error::Error;
pub use server::Server;

use std::net::SocketAddr;

pub use proto_core;

/// Configuration structure for building and launching a VPN server instance.
///
/// Cryptographic keys and network binding information used to initialize the
/// protocol server.
#[derive(Debug)]
pub struct ServerBuilder {
    /// Address to bind the server to (e.g., 0.0.0.0:781).
    pub addr: SocketAddr,

    /// Symmetric encryption key used for encryption.
    pub encryption_key: Vec<u8>,
    /// Signing key used for token authentication and message integrity.
    pub signing_key: Vec<u8>,
}
