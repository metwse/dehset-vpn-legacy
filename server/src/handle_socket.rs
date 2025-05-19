use crate::server::SharedState;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;

pub(crate) async fn handle_socket(
    (_tcp_stream, _remote_addr): (TcpStream, SocketAddr),
    _state: Arc<SharedState>,
) {
    todo!();
}
