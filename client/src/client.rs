use crate::{ClientBuilder, Error};
use crypto::symm::Aes128Cbc;
use proto_core::{
    SignedToken, random_bytes,
    sub_protocol2::handshake::{
        self, HandshakeAlert, HandshakeContentType, read_handshake_payload, write_handshake_payload,
    },
};
use tokio::net::TcpStream;
use tracing::{info, instrument, trace};

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

        client.handshake(self.token).await?;

        Ok(client)
    }
}

impl Client {
    #[instrument(skip(self, _token))]
    async fn handshake(&mut self, _token: SignedToken) -> Result<(), Error> {
        let client_hello = handshake::ClientHello {
            version: 0,
            encryption_algorithm: proto_core::EncryptionAlgorithm::Aes128CbcSha256,
            signature_algorithm: proto_core::SignatureAlgorithm::HmacSha256,
        };

        let payload = bincode::serde::encode_to_vec(&client_hello, bincode::config::standard())?;

        write_handshake_payload(
            &mut self.tcp_stream,
            handshake::HandshakeContentType::ClientHello,
            &payload,
        )
        .await?;

        info!("Sent client hello: {client_hello:?}");

        let (content_type, payload) = read_handshake_payload(&mut self.tcp_stream).await?;

        // The client expects the server's first payload to be a `ServerHello`
        // or `HandshakeAlert`.
        if content_type == HandshakeContentType::HandshakeAlert {
            let (alert, _): (HandshakeAlert, _) =
                bincode::serde::decode_from_slice(&payload, bincode::config::standard())?;
            return Err(Error::Handshake(alert));
        }
        if content_type != HandshakeContentType::ServerHello {
            return Err(Error::Handshake(HandshakeAlert::UnexpectedPayload {
                got: content_type,
                expected: vec![
                    HandshakeContentType::ServerHello,
                    HandshakeContentType::HandshakeAlert,
                ],
            }));
        }

        // TODO: decrypt
        let server_hello: (handshake::ServerHello, _) =
            bincode::serde::decode_from_slice(&payload, bincode::config::standard())
                .map_err(|_| HandshakeAlert::InvalidPayload)?;

        info!("Got server hello: {server_hello:?}");

        let client_random = random_bytes!(32);

        let finished = handshake::Finished {
            random: client_random,
        };

        // TODO: encrypt
        let payload = bincode::serde::encode_to_vec(&finished, bincode::config::standard())?;

        write_handshake_payload(
            &mut self.tcp_stream,
            handshake::HandshakeContentType::Finished,
            &payload,
        )
        .await?;

        info!("Sent finished: {finished:?}");

        info!("Handshake is done.");

        Ok(())
    }
}
