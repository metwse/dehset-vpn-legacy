use crate::{ClientBuilder, Error};
use crypto::symm::Aes128Cbc;
use tokio::net::TcpStream;

/// Internal VPN client struct.
pub struct Client {
    pub(crate) tcp_stream: TcpStream,
    pub(crate) encrypter: Aes128Cbc,
}

impl ClientBuilder {
    pub async fn try_build(self) -> Result<Client, Error> {
        let encrypter = Aes128Cbc::new(self.encryption_key);
        let tcp_stream = TcpStream::connect(self.addr).await?;

        Ok(Client {
            tcp_stream,
            encrypter,
        })
    }
}
