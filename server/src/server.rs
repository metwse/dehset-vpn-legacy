use crate::{Error, ServerBuilder, connection::Connection};
use crypto::{sign::Hs256, symm::Aes128Cbc};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, instrument, trace};

/// Internal VPN server struct holding shared state.
#[derive(Debug)]
#[must_use]
pub struct Server {
    pub(crate) shared_state: SharedState,
    pub(crate) tcp_listener: TcpListener,
}

#[derive(Debug)]
pub(crate) struct SharedState {
    pub(crate) _signer: Hs256,
    pub(crate) _encrypter: Aes128Cbc,
}

impl ServerBuilder {
    /// Consumes `self` and builds a [`Server`] instance.
    #[instrument(skip(self), fields(self.addr))]
    pub async fn try_build(self) -> Result<Server, Error> {
        let signer = Hs256::try_new(&self.signing_key)?;
        let encrypter = Aes128Cbc::new(self.encryption_key);
        let tcp_listener = TcpListener::bind(self.addr).await?;
        trace!(%self.addr, "Bind socket");

        Ok(Server {
            shared_state: SharedState {
                _signer: signer,
                _encrypter: encrypter,
            },
            tcp_listener,
        })
    }
}

impl Server {
    /// Serves the server forever.
    #[instrument(skip(self))]
    pub async fn serve(self) -> Result<(), Error> {
        let shared_state = Arc::new(self.shared_state);
        trace!("Serving the server");

        loop {
            let (tcp_stream, remote_addr) = self.tcp_listener.accept().await?;
            info!("Got connection from {remote_addr}");
            let state = Arc::clone(&shared_state);

            let mut connection = Connection {
                tcp_stream,
                _remote_addr: remote_addr,
                _state: state,
            };

            if let Err(handshake_alert) = connection.handshake().await {
                info!("Could not complete handshake: {handshake_alert:?}")
                // TODO: Send handshake alerts to the client.
            } else {
                // TODO: Handle socket.
            }

            info!("Lost connection {remote_addr}");
        }
    }
}
