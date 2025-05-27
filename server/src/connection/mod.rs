//! Utilities for managing client connections and handling sub-protocol layers.

mod handshake;

pub use handshake::do_handshake;

use crate::server::SharedState;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;

/// Represents a client connection to the server.
///
/// Wraps the underlying TCP stream, remote address, and shared server state.
pub struct Connection {
    pub(crate) _tcp_stream: TcpStream,
    pub(crate) _remote_addr: SocketAddr,
    pub(crate) _state: Arc<SharedState>,
}
