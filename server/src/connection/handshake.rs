use super::Connection;
use proto_core::{
    random_bytes,
    sub_protocol2::handshake::{
        self, HandshakeAlert, HandshakeContentType, read_handshake_payload, write_handshake_payload,
    },
};
use tracing::{info, instrument};

impl Connection {
    #[instrument(skip(self))]
    pub async fn handshake(&mut self) -> Result<(), HandshakeAlert> {
        let (content_type, payload) = read_handshake_payload(&mut self.tcp_stream).await?;

        // The server expects the client's first payload to be a `ClientHello`.
        if content_type != HandshakeContentType::ClientHello {
            return Err(HandshakeAlert::UnexpectedPayload {
                got: content_type,
                expected: vec![HandshakeContentType::ClientHello],
            });
        }

        let client_hello: (handshake::ClientHello, _) =
            bincode::serde::decode_from_slice(&payload, bincode::config::standard())
                .map_err(|_| HandshakeAlert::InvalidPayload)?;

        info!("Got client hello: {client_hello:?}");

        let server_random = random_bytes!(32);

        let server_hello = handshake::ServerHello {
            random: server_random,
        };
        let payload = bincode::serde::encode_to_vec(&server_hello, bincode::config::standard())
            .map_err(|_| HandshakeAlert::InvalidPayload)?;

        // TODO: encrypt
        write_handshake_payload(
            &mut self.tcp_stream,
            HandshakeContentType::ServerHello,
            &payload,
        )
        .await?;

        info!("Send server hello: {server_hello:?}");

        // TODO: decrypt
        let (content_type, payload) = read_handshake_payload(&mut self.tcp_stream).await?;

        // A `Finished` payload is expected.
        if content_type != HandshakeContentType::Finished {
            return Err(HandshakeAlert::UnexpectedPayload {
                got: content_type,
                expected: vec![HandshakeContentType::Finished],
            });
        }

        let finished: (handshake::Finished, _) =
            bincode::serde::decode_from_slice(&payload, bincode::config::standard())
                .map_err(|_| HandshakeAlert::InvalidPayload)?;

        info!("Got finished: {finished:?}");

        info!("Handshake is done.");

        Ok(())
    }
}
