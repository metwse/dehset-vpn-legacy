use crate::{ClientBuilder, Error, connection::do_handshake};
use crypto::symm::Aes128Cbc;
use tokio::net::TcpStream;
use tracing::{instrument, trace};

/// Internal VPN client struct.
pub struct Client {
    pub(crate) _tcp_stream: TcpStream,
    pub(crate) _encrypter: Aes128Cbc,
}

impl ClientBuilder {
    #[instrument(skip(self))]
    pub async fn try_build(self) -> Result<Client, Error> {
        let encrypter = Aes128Cbc::new(self.encryption_key);
        let mut tcp_stream = TcpStream::connect(self.addr).await?;

        trace!("Connected to {tcp_stream:?}");

        let (mut r, mut w) = tcp_stream.split();

        do_handshake(&mut r, &mut w).await?;

        let client = Client {
            _tcp_stream: tcp_stream,
            _encrypter: encrypter,
        };

        Ok(client)
    }
}

impl Client {}
