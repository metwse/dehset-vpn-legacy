use crate::{Error, server::SharedState};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;
use tracing::instrument;

#[instrument(skip(_state, _tcp_stream))]
pub(crate) async fn handle_socket(
    (_tcp_stream, remote_addr): (&mut TcpStream, SocketAddr),
    _state: Arc<SharedState>,
) -> Result<(), Error> {
    Ok(())
}
