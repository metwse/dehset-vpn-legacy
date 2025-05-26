use crate::{Error, server::SharedState};
use proto_core::{ContentType, HandshakeContentType, Payload, handshake, random_bytes};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{debug, info, instrument};

async fn read_payload(stream: &mut TcpStream) -> Result<Payload, Error> {
    let content_type = if let Ok(content_type) = ContentType::try_from(stream.read_u8().await?) {
        content_type
    } else {
        return Err(Error::Handshake("invalid content type"));
    };
    let content_length = stream.read_u16().await?;
    debug!("Receive {content_type:?} ({content_length})");

    let mut payload = Payload {
        content_type,
        content_length,
        payload: vec![0; content_length as usize],
    };

    stream.read_exact(&mut payload.payload).await?;

    Ok(payload)
}

#[instrument(skip(state, tcp_stream))]
pub(crate) async fn handle_socket(
    (mut tcp_stream, remote_addr): (TcpStream, SocketAddr),
    state: Arc<SharedState>,
) -> Result<(), Error> {
    info!("Awaiting handshake...");
    let payload = read_payload(&mut tcp_stream).await?;

    if payload.content_type != ContentType::Handshake
        || payload.content_length < 1
        || payload.payload[0] != HandshakeContentType::ClientHello as u8
    {
        return Ok(());
    }

    let (client_hello, _): (handshake::ClientHello, _) =
        bincode::serde::decode_from_slice(&payload.payload[1..], bincode::config::standard())?;
    info!("Accept {:?}", client_hello);

    let server_random = random_bytes!(32);

    let server_hello = handshake::ServerHello {
        accept: true,
        random: server_random,
    };

    let mut encrypted_server_hello =
        bincode::serde::encode_to_vec(&server_hello, bincode::config::standard())?;

    encrypted_server_hello = state.encrypter.encrypt(None, &encrypted_server_hello)?;

    tcp_stream.write_u8(ContentType::Handshake as u8).await?;
    tcp_stream
        .write_u16(encrypted_server_hello.len() as u16 + 1)
        .await?;
    tcp_stream
        .write_u8(HandshakeContentType::ServerHello as u8)
        .await?;
    tcp_stream.write_all(&encrypted_server_hello).await?;
    info!("Sent {:?}", server_hello);

    let authenticate = read_payload(&mut tcp_stream).await?;
    if authenticate.content_type != ContentType::Handshake
        || authenticate.content_length < 1
        || authenticate.payload[0] != HandshakeContentType::Authenticate as u8
    {
        return Ok(());
    }
    let authenticate = state
        .encrypter
        .decrypt(Some(&server_random), &authenticate.payload[1..])?;

    let (authenticate, _): (handshake::Authenticate, _) =
        bincode::serde::decode_from_slice(&authenticate[..], bincode::config::standard()).unwrap();
    info!("Authenticate {:?}", authenticate);

    Ok(())
}
