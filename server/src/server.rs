use crate::{Error, ServerBuilder, handle_socket::handle_socket, handshake::handshake};
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
    pub(crate) signer: Hs256,
    pub(crate) encrypter: Aes128Cbc,
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
                signer: signer,
                encrypter: encrypter,
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
            let (mut tcp_stream, remote_addr) = self.tcp_listener.accept().await?;
            info!("Got connection from {remote_addr}");
            let state = Arc::clone(&shared_state);
            let handshake = handshake((&mut tcp_stream, remote_addr), Arc::clone(&state)).await;
            if let Err(_handshake_error) = handshake {
                // TODO: Send handshake error alerts to the client
            } else {
                handle_socket((&mut tcp_stream, remote_addr), Arc::clone(&state))
                    .await
                    .ok();
            }
            info!("Lost connection {remote_addr}");
        }
    }
}
