mod handshake;

use crate::server::SharedState;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;

/// Client connection to the server.
pub struct Connection {
    pub tcp_stream: TcpStream,
    pub _remote_addr: SocketAddr,
    pub _state: Arc<SharedState>,
}
