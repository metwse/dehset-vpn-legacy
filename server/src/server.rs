use super::handle_socket::handle_socket;
use crate::{Error, ServerBuilder};
use crypto::{sign::Hs256, symm::Aes128Cbc};
use std::sync::Arc;
use tokio::net::TcpListener;

/// Internal VPN server struct holding shared state.
pub struct Server {
    pub(crate) shared_state: SharedState,
    pub(crate) tcp_listener: TcpListener,
}

pub(crate) struct SharedState {
    pub(crate) _signer: Hs256,
    pub(crate) _encrypter: Aes128Cbc,
}

impl ServerBuilder {
    /// Consumes `self` and builds a [`Server`] instance.
    pub async fn try_build(self) -> Result<Server, Error> {
        let signer = Hs256::try_new(&self.signing_key)?;
        let encrypter = Aes128Cbc::new(self.encryption_key);
        let tcp_listener = TcpListener::bind(self.addr).await?;

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
    pub async fn serve(self) -> Result<(), Error> {
        let shared_state = Arc::new(self.shared_state);

        loop {
            let (tcp_stream, remote_addr) = self.tcp_listener.accept().await?;
            let state = Arc::clone(&shared_state);
            handle_socket((tcp_stream, remote_addr), state).await;
        }
    }
}
