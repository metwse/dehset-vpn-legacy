use crate::{
    Error, ServerBuilder,
    connection::{Connection, do_handshake},
};
use crypto::{sign::Hs256, symm::Aes128CbcSha256, tls::SymmTls};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, instrument, trace};

/// Internal VPN server struct holding shared state.
#[derive(Debug)]
#[must_use]
pub struct Server {
    pub(crate) shared_state: SharedState,
    pub(crate) tcp_listener: TcpListener,
    pub(crate) encrypter: Aes128CbcSha256,
}

#[derive(Debug)]
pub(crate) struct SharedState {
    pub(crate) _signer: Hs256,
}

impl ServerBuilder {
    /// Consumes `self` and builds a [`Server`] instance.
    #[instrument(skip(self), fields(self.addr))]
    pub async fn try_build(self) -> Result<Server, Error> {
        let signer = Hs256::try_new(&self.signing_key)?;
        let encrypter = Aes128CbcSha256::new(self.encryption_key);
        let tcp_listener = TcpListener::bind(self.addr).await?;
        trace!(%self.addr, "Bind socket");

        Ok(Server {
            shared_state: SharedState { _signer: signer },
            encrypter,
            tcp_listener,
        })
    }
}

impl Server {
    /// Serves the server forever.
    #[instrument(skip(self))]
    pub async fn serve(self) -> Result<(), Error> {
        let encrypter = Arc::new(self.encrypter);
        let shared_state = Arc::new(self.shared_state);
        trace!("Serving the server");

        loop {
            let (mut tcp_stream, remote_addr) = self.tcp_listener.accept().await?;
            info!("Got connection from {remote_addr}");
            tokio::spawn({
                let encrpter = Arc::clone(&encrypter);
                let shared_state = Arc::clone(&shared_state);
                async move {
                    let state = Arc::clone(&shared_state);

                    let (mut r, mut w) = tcp_stream.split();

                    match do_handshake(&mut r, &mut w).await {
                        Err(handshake_alert) => {
                            info!("Could not complete handshake: {handshake_alert:?}");
                            // TODO: Send handshake alerts to the client.
                        }
                        Ok((server_random, client_random)) => {
                            let tls = SymmTls::new((server_random, client_random), encrpter);

                            let mut _connection = Connection {
                                _tcp_stream: tcp_stream,
                                _state: state,
                                _tls: tls,
                            };
                            // TODO: Handle socket.
                        }
                    }
                }
            });
            info!("Lost connection {remote_addr}");
        }
    }
}
