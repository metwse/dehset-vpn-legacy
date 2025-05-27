use crate::{ClientBuilder, Error, connection::do_handshake};
use crypto::symm::Aes128Cbc;
use tokio::net::TcpStream;
use tracing::{instrument, trace};

/// Internal VPN client struct.
pub struct Client {
    pub(crate) tcp_stream: TcpStream,
    pub(crate) _encrypter: Aes128Cbc,
}

impl ClientBuilder {
    #[instrument(skip(self))]
    pub async fn try_build(self) -> Result<Client, Error> {
        let encrypter = Aes128Cbc::new(self.encryption_key);
        let tcp_stream = TcpStream::connect(self.addr).await?;

        trace!("Connected to {tcp_stream:?}");

        let mut client = Client {
            tcp_stream,
            _encrypter: encrypter,
        };

        do_handshake(&mut client.tcp_stream).await?;

        Ok(client)
    }
}

impl Client {}
