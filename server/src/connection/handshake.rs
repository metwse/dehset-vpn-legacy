use proto_core::{
    handshake::{
        self, HandshakeAlert, HandshakeContentType, read_handshake_payload, write_handshake_payload,
    },
    random_bytes,
};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{info, instrument, trace};

/// Server-side implementation of the handshake protocol (version 0.1).
///
/// Returns a [`HandshakeAlert`] if an error occurs, which should then be sent
/// back to the client and terminate the connection.
#[instrument(skip(r, w))]
pub async fn do_handshake<R: Unpin + AsyncRead, W: Unpin + AsyncWrite>(
    r: &mut R,
    w: &mut W,
) -> Result<(), HandshakeAlert> {
    let (content_type, payload) = read_handshake_payload(r).await?;

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
    write_handshake_payload(w, HandshakeContentType::ServerHello, &payload).await?;

    trace!("Send server hello: {server_hello:?}");

    // TODO: decrypt
    let (content_type, payload) = read_handshake_payload(r).await?;

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

#[cfg(test)]
mod tests {
    use super::do_handshake;
    use proto_core::{
        handshake::{self, HandshakeAlert, HandshakeContentType, read_handshake_payload},
        random_bytes,
    };
    use testutil::{DynResult, send_handshake_payload};
    use tokio::io::simplex;

    #[tokio::test]
    async fn expected_client_hello() -> DynResult<()> {
        let (mut sr, mut cw) = simplex(u16::MAX as usize);
        let (_, mut sw) = simplex(u16::MAX as usize);

        send_handshake_payload!(
            &mut cw,
            HandshakeContentType::Finished,
            handshake::Finished {
                random: random_bytes!(32)
            }
        );

        if let Err(HandshakeAlert::UnexpectedPayload { .. }) = do_handshake(&mut sr, &mut sw).await
        {
            Ok(())
        } else {
            panic!("Expected HandshakeAlert::UnexpectedPayload")
        }
    }

    #[tokio::test]
    async fn server_hello() -> DynResult<()> {
        let (mut sr, mut cw) = simplex(u16::MAX as usize);
        let (mut cr, mut sw) = simplex(u16::MAX as usize);

        send_handshake_payload!(
            &mut cw,
            HandshakeContentType::ClientHello,
            handshake::ClientHello {
                version: 0,
                encryption_algorithm: proto_core::EncryptionAlgorithm::Aes128CbcSha256,
                signature_algorithm: proto_core::SignatureAlgorithm::HmacSha256,
            }
        );

        tokio::spawn(async move { do_handshake(&mut sr, &mut sw).await.unwrap() });

        let (content_type, _) = read_handshake_payload(&mut cr).await.unwrap();

        assert_eq!(content_type, handshake::HandshakeContentType::ServerHello);

        Ok(())
    }

    #[tokio::test]
    async fn expected_finished() -> DynResult<()> {
        let (mut sr, mut cw) = simplex(u16::MAX as usize);
        let (mut cr, mut sw) = simplex(u16::MAX as usize);

        send_handshake_payload!(
            &mut cw,
            HandshakeContentType::ClientHello,
            handshake::ClientHello {
                version: 0,
                encryption_algorithm: proto_core::EncryptionAlgorithm::Aes128CbcSha256,
                signature_algorithm: proto_core::SignatureAlgorithm::HmacSha256,
            }
        );
        send_handshake_payload!(
            &mut cw,
            // The content type should be finished.
            HandshakeContentType::ServerHello,
            handshake::Finished {
                random: random_bytes!(32)
            }
        );

        let task = tokio::spawn(async move { do_handshake(&mut sr, &mut sw).await.unwrap() });
        let (content_type, _) = read_handshake_payload(&mut cr).await.unwrap();

        assert_eq!(content_type, handshake::HandshakeContentType::ServerHello);

        if task.await.is_err() {
            Ok(())
        } else {
            panic!("Should err")
        }
    }
}
