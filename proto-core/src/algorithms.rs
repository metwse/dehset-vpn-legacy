//! Encryption and signing algorithms used in the protocol.
//!
//! The cryptographic primitives available to both clients and servers are
//! defined in these enums.

use serde::{Deserialize, Serialize};

/// Supported encryption algorithms. Currently only symmetric algorithms are
/// supported.
#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    /// AES-128 in CBC mode with SHA-256 for integrity.
    Aes128CbcSha256,
}

/// Supported signature algorithms.
#[derive(Debug, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    /// HMAC with SHA-256.
    HmacSha256,
}
