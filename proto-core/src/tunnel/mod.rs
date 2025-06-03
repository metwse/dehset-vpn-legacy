mod error;

pub use error::TunnelError;

use crate::tls_provider::TlsProvider;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::Mutex,
};

pub const MAX_PAYLOAD_SIZE: usize = 2usize.pow(24) - 4;

/// VPN tunnel that sends and receives payloads over an encrypted channel in
/// ordered manner.
///
/// The tunnel handles encrypted communication using the provided
/// [`TlsProvider`], ensuring data confidentiality and integrity between
/// endpoints.
pub struct Tunnel<R, W, T> {
    pub r: Mutex<R>,
    pub w: Mutex<W>,
    pub tls: T,
}

impl<R, W, T> Tunnel<R, W, T>
where
    W: Unpin + AsyncWrite,
    T: TlsProvider,
{
    pub async fn send(&self, payload: &[u8]) -> Result<(), TunnelError> {
        let encrypted = self.tls.encrypt(payload).map_err(|_| TunnelError::Crypto)?;

        if encrypted.len() > MAX_PAYLOAD_SIZE {
            return Err(TunnelError::PayloadTooLarge);
        }

        let mut w = self.w.lock().await;

        w.write_u32(encrypted.len() as u32).await?;
        w.write_all(&encrypted).await?;

        Ok(())
    }
}

impl<R, W, T> Tunnel<R, W, T>
where
    R: Unpin + AsyncRead,
    T: TlsProvider,
{
    pub async fn recv(&self) -> Result<Vec<u8>, TunnelError> {
        let mut r = self.r.lock().await;

        let content_lenght = r.read_u32().await? as usize;
        if content_lenght > MAX_PAYLOAD_SIZE {
            return Err(TunnelError::PayloadTooLarge);
        }

        let mut payload = vec![0; content_lenght];

        r.read_exact(&mut payload).await?;

        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_PAYLOAD_SIZE, Tunnel};
    use crate::{random_bytes, tls_provider::MockTls};
    use testutil::DynResult;
    use tokio::{io::simplex, sync::Mutex};

    #[tokio::test]
    pub async fn tunnel() -> DynResult<()> {
        let (r, w) = simplex(usize::MAX);

        let tunnel = Tunnel {
            r: Mutex::new(r),
            w: Mutex::new(w),
            tls: MockTls {},
        };

        let random = random_bytes!(u16::MAX as usize);
        let zero = vec![0; MAX_PAYLOAD_SIZE];

        tunnel.send(&random).await?;
        tunnel.send(&zero).await?;
        assert_eq!(tunnel.recv().await?, random);
        assert_eq!(tunnel.recv().await?, zero);

        Ok(())
    }
}
