mod message_queue;
mod error;

pub use message_queue::MessageQueue;
pub use error::TunnelError;

use crate::TlsProvider;
use tokio::io::{AsyncRead, AsyncWrite};

/// VPN tunnel that sends and receives payloads over an encrypted channel in
/// ordered manner.
///
/// The tunnel handles encrypted communication using the provided
/// [`TlsProvider`], ensuring data confidentiality and integrity between
/// endpoints.
pub struct Tunnel<R, W, T> {
    pub r: R,
    pub w: W,
    pub tls: T,
}

impl<R, W, T> Tunnel<R, W, T>
where
    R: Unpin + AsyncRead,
    T: TlsProvider,
{
    pub async fn send(&self, _payload: Vec<u8>) -> Result<(), TunnelError> {
        todo!()
    }
}

impl<R, W, T> Tunnel<R, W, T>
where
    W: Unpin + AsyncWrite,
    T: TlsProvider,
{
    pub async fn recv(&self) -> Result<Vec<u8>, TunnelError> {
        todo!()
    }
}
