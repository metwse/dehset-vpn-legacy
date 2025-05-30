//! Utilities for managing client connections and handling sub-protocol layers.

mod handshake;

pub use handshake::do_handshake;
use proto_core::tls_provider::TlsProvider;

use crate::server::SharedState;
use std::sync::Arc;
use tokio::net::TcpStream;

/// Represents a client connection to the server.
///
/// Wraps the underlying TCP stream, remote address, and shared server state.
pub struct Connection<T: TlsProvider> {
    pub(crate) _tcp_stream: TcpStream,
    pub(crate) _state: Arc<SharedState>,
    pub(crate) _tls: T,
}
