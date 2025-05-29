use crate::{connection::do_handshake, ClientBuilder, Error};
use crypto::{symm::Aes128CbcSha256, tls::SymmTls};
use tokio::net::TcpStream;
use tracing::{instrument, trace};
use std::sync::Arc;

/// Internal VPN client struct.
pub struct Client {
    pub(crate) _tcp_stream: TcpStream,
    pub(crate) _tls: SymmTls,
}

impl ClientBuilder {
    #[instrument(skip(self))]
    pub async fn try_build(self) -> Result<Client, Error> {
        let encrypter = Aes128CbcSha256::new(self.encryption_key);
        let mut tcp_stream = TcpStream::connect(self.addr).await?;

        trace!("Connected to {tcp_stream:?}");

        let (mut r, mut w) = tcp_stream.split();

        let (server_random, client_random) = do_handshake(&mut r, &mut w).await?;
        let tls = SymmTls::new((server_random, client_random), Arc::new(encrypter));

        let client = Client {
            _tcp_stream: tcp_stream,
            _tls: tls
        };

        Ok(client)
    }
}

impl Client {}
