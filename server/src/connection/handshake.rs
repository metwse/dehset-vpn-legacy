use proto_core::{
    random_bytes,
    sub_protocol2::handshake::{
        self, HandshakeAlert, HandshakeContentType, read_handshake_payload, write_handshake_payload,
    },
};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{info, instrument, trace};

/// Server-side implementation of the handshake protocol (version 0.1).
///
/// Returns a [`HandshakeAlert`] if an error occurs, which should then be sent
/// back to the client and terminate the connection.
#[instrument(skip(rw))]
pub async fn do_handshake<RW: Unpin + AsyncRead + AsyncWrite>(
    rw: &mut RW,
) -> Result<(), HandshakeAlert> {
    let (content_type, payload) = read_handshake_payload(rw).await?;

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

    trace!("Got client hello: {client_hello:?}");

    let server_random = random_bytes!(32);

    let server_hello = handshake::ServerHello {
        random: server_random,
    };
    let payload = bincode::serde::encode_to_vec(&server_hello, bincode::config::standard())
        .map_err(|_| HandshakeAlert::InvalidPayload)?;

    // TODO: encrypt
    write_handshake_payload(rw, HandshakeContentType::ServerHello, &payload).await?;

    trace!("Send server hello: {server_hello:?}");

    // TODO: decrypt
    let (content_type, payload) = read_handshake_payload(rw).await?;

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

    trace!("Got finished: {finished:?}");

    info!("Handshake is done.");

    Ok(())
}
