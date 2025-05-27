use crate::Error;
use proto_core::{
    random_bytes,
    sub_protocol2::handshake::{
        self, HandshakeAlert, HandshakeContentType, read_handshake_payload, write_handshake_payload,
    },
};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{info, instrument, trace};

/// Client-side implementation of the handshake protocol (version 0.1).
///
/// Returns an [`Error`] if an error occurs, then the connection should
/// terminated.
#[instrument(skip(rw))]
pub async fn do_handshake<RW: Unpin + AsyncRead + AsyncWrite>(rw: &mut RW) -> Result<(), Error> {
    let client_hello = handshake::ClientHello {
        version: 0,
        encryption_algorithm: proto_core::EncryptionAlgorithm::Aes128CbcSha256,
        signature_algorithm: proto_core::SignatureAlgorithm::HmacSha256,
    };

    let payload = bincode::serde::encode_to_vec(&client_hello, bincode::config::standard())?;

    write_handshake_payload(rw, handshake::HandshakeContentType::ClientHello, &payload).await?;

    trace!("Sent client hello: {client_hello:?}");

    let (content_type, payload) = read_handshake_payload(rw).await?;

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

    trace!("Got server hello: {server_hello:?}");

    let client_random = random_bytes!(32);

    let finished = handshake::Finished {
        random: client_random,
    };

    // TODO: encrypt
    let payload = bincode::serde::encode_to_vec(&finished, bincode::config::standard())?;

    write_handshake_payload(rw, handshake::HandshakeContentType::Finished, &payload).await?;

    trace!("Sent finished: {finished:?}");

    info!("Handshake is done.");

    Ok(())
}
