use crate::Error;
use proto_core::{
    random_bytes,
    sub_protocol::handshake::{
        self, HandshakeAlert, HandshakeContentType, read_handshake_payload, write_handshake_payload,
    },
};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{info, instrument, trace};

/// Client-side implementation of the handshake protocol (version 0.1).
///
/// Returns an [`Error`] if an error occurs, then the connection should
/// terminated.
#[instrument(skip(r, w))]
pub async fn do_handshake<R: Unpin + AsyncRead, W: Unpin + AsyncWrite>(
    r: &mut R,
    w: &mut W,
) -> Result<([u8; 32], [u8; 32]), Error> {
    let client_hello = handshake::ClientHello {
        version: 0,
        encryption_algorithm: proto_core::algorithms::EncryptionAlgorithm::Aes128CbcSha256,
        signature_algorithm: proto_core::algorithms::SignatureAlgorithm::HmacSha256,
    };

    let payload = bincode::serde::encode_to_vec(&client_hello, bincode::config::standard())?;

    write_handshake_payload(w, handshake::HandshakeContentType::ClientHello, &payload).await?;

    trace!("Sent client hello: {client_hello:?}");

    let (content_type, payload) = read_handshake_payload(r).await?;

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
    let (server_hello, _): (handshake::ServerHello, _) =
        bincode::serde::decode_from_slice(&payload, bincode::config::standard())
            .map_err(|_| HandshakeAlert::InvalidPayload)?;

    trace!("Got server hello: {server_hello:?}");

    let client_random = random_bytes!(32);

    let finished = handshake::Finished {
        random: client_random,
    };

    // TODO: encrypt
    let payload = bincode::serde::encode_to_vec(&finished, bincode::config::standard())?;

    write_handshake_payload(w, handshake::HandshakeContentType::Finished, &payload).await?;

    trace!("Sent finished: {finished:?}");

    info!("Handshake is done.");

    Ok((server_hello.random, client_random))
}

#[cfg(test)]
mod tests {
    use super::do_handshake;
    use proto_core::{
        random_bytes,
        sub_protocol::handshake::{self, HandshakeContentType, read_handshake_payload},
    };
    use testutil::{DynResult, send_handshake_payload};
    use tokio::io::simplex;

    #[tokio::test]
    async fn expect_server_hello() -> Result<(), DynResult<()>> {
        let (mut sr, mut cw) = simplex(u16::MAX as usize);
        let (mut cr, mut sw) = simplex(u16::MAX as usize);

        let task = tokio::spawn(async move { do_handshake(&mut cr, &mut cw).await.unwrap() });

        let (content_type, _) = read_handshake_payload(&mut sr).await.unwrap();

        assert_eq!(content_type, HandshakeContentType::ClientHello);

        send_handshake_payload!(
            &mut sw,
            HandshakeContentType::ClientHello,
            handshake::Finished {
                random: random_bytes!(32),
            }
        );

        if task.await.is_err() {
            Ok(())
        } else {
            panic!("Expected Error::Handshake(alert)")
        }
    }
}
