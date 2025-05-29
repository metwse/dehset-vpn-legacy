//! Secure encryption/signing layer established as a result of the handshake
//! sub-protocol.

//! A successful handshake returns an implementation of [`TlsProvider`].
//! This is used by application and VPN layers to secure traffic.

/// Provides encryption and decryption methods used after a successful handshake.
/// This trait abstracts over the secure channel implementation
/// (e.g., symmetric or asymmetric encryption).
///
/// The [`TlsProvider`] implementation may include internal mutability, but it
/// must be thread-safe, typically achieved through synchronization primitives
/// such as [`std::sync::Mutex`] or [`std::sync::RwLock`].
pub trait TlsProvider {
    type Error;

    /// Encrypts raw data to be safely transmitted over the network.
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;

    /// Decrypts received encrypted data back to its original form.
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Self::Error>;
}
