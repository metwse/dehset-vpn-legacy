use crate::{ClientBuilder, Error};
use crypto::symm::Aes128Cbc;
use proto_core::{
    ContentType, HandshakeContentType, SignedToken,
    handshake::{Authenticate, ClientHello, ServerHello},
    random_bytes,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{debug, info, instrument, trace};

/// Internal VPN client struct.
pub struct Client {
    pub(crate) tcp_stream: TcpStream,
    pub(crate) encrypter: Aes128Cbc,
}

impl ClientBuilder {
    #[instrument(skip(self))]
    pub async fn try_build(self) -> Result<Client, Error> {
        let encrypter = Aes128Cbc::new(self.encryption_key);
        let tcp_stream = TcpStream::connect(self.addr).await?;

        trace!("Connected to {tcp_stream:?}");

        let mut client = Client {
            tcp_stream,
            encrypter,
        };

        client.handshake(self.token).await?;

        Ok(client)
    }
}

impl Client {
    #[instrument(skip(self, token))]
    async fn handshake(&mut self, token: SignedToken) -> Result<(), Error> {
        // ClientHello
        let client_hello = ClientHello {
            version: 0,
            encryption_algorithm: proto_core::EncryptionAlgorithm::Aes128CbcSha256,
        };

        let client_hello =
            bincode::serde::encode_to_vec(client_hello, bincode::config::standard())?;

        self.send_handshake_payload(HandshakeContentType::ClientHello, &client_hello)
            .await?;
        info!("Sent client hello.");
        // end of ClientHello

        // ServerHello
        let (content_type, server_hello) = self.recv_payload().await?;
        if content_type != ContentType::Handshake {
            return Err(Error::Handshake("invalid content type"));
        }

        let handshake_content_type = server_hello[0];
        let server_hello = self.encrypter.decrypt(None, &server_hello[1..])?;
        if handshake_content_type != HandshakeContentType::ServerHello as u8 {
            return Err(Error::Handshake("invalid content type"));
        }

        let server_hello: ServerHello =
            bincode::serde::decode_from_slice(&server_hello, bincode::config::standard())?.0;

        if !server_hello.accept {
            return Err(Error::Handshake("connection is not accepted"));
        }

        let server_random = server_hello.random;
        info!("Got server hello: {server_hello:?}");
        // end of ServerHello

        // Authenticate
        let authenticate = self.encrypter.encrypt(
            Some(&server_random),
            &bincode::serde::encode_to_vec(
                Authenticate {
                    token,
                    random: random_bytes!(32),
                },
                bincode::config::standard(),
            )?,
        )?;

        self.send_handshake_payload(HandshakeContentType::Authenticate, &authenticate)
            .await?;
        info!("Sent authenticate.");
        // end of Authenticate

        Ok(())
    }

    async fn recv_payload(&mut self) -> Result<(ContentType, Vec<u8>), Error> {
        let content_type =
            if let Ok(content_type) = ContentType::try_from(self.tcp_stream.read_u8().await?) {
                content_type
            } else {
                return Err(Error::Handshake("invalid content type"));
            };
        let content_length = self.tcp_stream.read_u16().await?;
        let mut payload = vec![0; content_length as usize];
        self.tcp_stream.read_exact(&mut payload).await?;
        debug!("Receive {content_type:?} ({content_length})");

        Ok((content_type, payload))
    }

    async fn send_handshake_payload(
        &mut self,
        handshake_content_type: HandshakeContentType,
        data: &[u8],
    ) -> Result<(), Error> {
        self.tcp_stream
            .write_u8(ContentType::Handshake as u8)
            .await?;
        self.tcp_stream.write_u16((data.len() + 1) as u16).await?;
        self.tcp_stream
            .write_u8(handshake_content_type as u8)
            .await?;
        self.tcp_stream.write_all(data).await?;

        Ok(())
    }
}
