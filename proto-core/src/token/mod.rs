//! Authentication token model for protocol-level permission control.
//!
//! Each [`Token`] represents an identity with an associated set of permissions
//! (`scope`), a permission level (`level`), and optional tagging for
//! fine-grained control.
//!
//! The [`TokenScope`] enum defines hierarchical capabilities, from simple
//! port forwarding to administrative actions such as issuing and revoking
//! tokens. Tag-based filtering enables scoped access via [`TokenTag`]s,
//! which can be either string literals or regular expressions.
//!
//! This design supports scalable and secure delegation of responsibilities
//! between nodes with varying trust levels.

mod error;

pub use error::TokenError;

use crate::algorithms::SignatureAlgorithm;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

/// Node ID Token.
#[derive(Serialize, Deserialize)]
pub struct Token {
    /// Subject: Token ID, used for token revocation.
    pub sub: u64,
    /// Issued at: Unix timestamp indicating when the token was issued.
    pub iat: u64,
    /// Expiration time: Unix timestamp indicating when the token expires.
    pub exp: u64,

    /// Name: An informative label for identifying the token.
    pub name: String,

    /// Tags assigned to nodes with the [`TokenScope::ForwardPort`] permission.
    pub tags: Vec<String>,

    /// List of permissions associated with the token.
    pub scope: Vec<TokenScope>,

    /// Permission level. Nodes with a lower permission level can neither
    /// request ports nor manage tokens of higher-level nodes.
    pub level: u64,
}

/// Signed [`Token`] variant.
#[derive(Serialize, Deserialize)]
pub struct SignedToken {
    pub token: Token,
    pub signature: Vec<u8>,
    pub signature_algorithm: SignatureAlgorithm,
}

/// Token permissions.
#[derive(Serialize, Deserialize)]
pub enum TokenScope {
    /// The lowest permission level. A salvage node can only forward ports.
    ForwardPort,

    /// Permission for a client to request ports from tagged nodes.
    RequestPort {
        /// Tags of the nodes from which ports can be requested.
        tags: Vec<TokenTag>,
        /// Range of allowable ports that can be requested.
        ports: Vec<RangeInclusive<u16>>,
    },

    /// Administrative permission level that allows issuing and revoking tokens.
    Super,
}

/// Token tag enum.
#[derive(Serialize, Deserialize)]
pub enum TokenTag {
    /// A tag defined by a literal string.
    StringLiteral(String),
    /// A tag defined by a regular expression.
    Regex(String),
}

impl Token {
    /// Encode token into binary format. Used in signature verification.
    pub fn encode(&self) -> Result<Vec<u8>, TokenError> {
        Ok(bincode::serde::encode_to_vec(
            self,
            bincode::config::standard(),
        )?)
    }
}
