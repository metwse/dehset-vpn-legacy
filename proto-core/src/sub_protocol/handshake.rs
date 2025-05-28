//! Pre-encryption handshake messages.
//!
//! The current protocol version implements a fairly simple and straightforward
//! symmetric encryption mechanism. Nodes and servers are assumed to already
//! share knowledge of the symmetric encryption key (e.g., AES).
//!
//! All handshake payloads follow the structure:
//! ```text
//! bytes
//!  0, 1   content_length
//!     2   handshake_content_type
//!   3..   payload data
//! ```

use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{EncryptionAlgorithm, SignatureAlgorithm};

/// Content type of the handshake payloads.
///
/// Currently, these handshake layers are implemented for symmetric encryption.
/// The protocol is subject to change with future asymmetric encryption support.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HandshakeContentType {
    /// Alert, warning, or error message.
    HandshakeAlert = 0,
    /// Initial payload sent by the client.
    ClientHello = 1,
    /// Server’s response to the client hello.
    ServerHello = 2,
    /// Final handshake message used to derive the IV.
    Finished = 3,
}

impl std::convert::TryFrom<u8> for HandshakeContentType {
    type Error = HandshakeAlert;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::HandshakeAlert),
            1 => Ok(Self::ClientHello),
            2 => Ok(Self::ServerHello),
            3 => Ok(Self::Finished),
            _ => Err(HandshakeAlert::UnknownContentType),
        }
    }
}

impl From<std::io::Error> for HandshakeAlert {
    fn from(_: std::io::Error) -> Self {
        HandshakeAlert::IoError
    }
}

/// Alert messages used during the handshake process to signal failures.
#[derive(Debug, Serialize, Deserialize)]
pub enum HandshakeAlert {
    /// The client's protocol version is not supported by the server.
    ///
    /// Contains the range(s) of versions the server does support.
    UnsupportedVersion {
        supported_versions: Vec<RangeInclusive<u16>>,
    },
    /// The encryption, signature, or token algorithm requested by the client
    /// is not supported by the server.
    ///
    /// `details` typically contains the name of the unsupported algorithm.
    UnsupportedAlgorithm { details: String },

    /// The received handshake payload did not match the expected type(s).
    ///
    /// Contains a list of acceptable [`HandshakeContentType`] variants that
    /// were expected at this point.
    UnexpectedPayload {
        got: HandshakeContentType,
        expected: Vec<HandshakeContentType>,
    },

    /// The content type of the payload is unrecognized or unsupported.
    UnknownContentType,

    /// An unspecified I/O error. Typically caused by network corruption.
    IoError,
    /// Indicates that the payload is invalid or corrupted.
    ///
    /// All encoding and decoding errors are converted into this type.
    InvalidPayload,
}

/// Initial payload sent by the client.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientHello {
    pub version: u16,
    pub encryption_algorithm: EncryptionAlgorithm,
    pub signature_algorithm: SignatureAlgorithm,
}

/// Server's encrypted response to the client hello.
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerHello {
    pub random: [u8; 32],
}

/// Final step of the handshake, where the client sends a random vector to
/// the server for IV (Initialization Vector) derivation or session key
/// agreement.
#[derive(Debug, Serialize, Deserialize)]
pub struct Finished {
    pub random: [u8; 32],
}

/// Reads a handshake payload from the TCP stream.
pub async fn read_handshake_payload<R: Unpin + AsyncRead>(
    r: &mut R,
) -> Result<(HandshakeContentType, Vec<u8>), HandshakeAlert> {
    let content_length = r.read_u16().await?;

    let handshake_content_type = r.read_u8().await?;
    let handshake_content_type = HandshakeContentType::try_from(handshake_content_type)?;

    let mut payload = vec![0; content_length as usize];
    r.read_exact(&mut payload).await?;

    Ok((handshake_content_type, payload))
}

/// Writes the handshake payload to the TCP stream.
pub async fn write_handshake_payload<W: Unpin + AsyncWrite>(
    w: &mut W,
    handshake_content_type: HandshakeContentType,
    payload: &[u8],
) -> Result<(), HandshakeAlert> {
    w.write_u16(payload.len() as u16).await?;
    w.write_u8(handshake_content_type as u8).await?;
    w.write_all(payload).await?;

    Ok(())
}
