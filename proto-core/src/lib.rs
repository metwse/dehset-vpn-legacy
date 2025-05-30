//! Core protocol definitions and shared types for the VPN system.
//!
//! Low-level message structures, token formats, and sub-protocols used across
//! all VPN-related components are defined in this crate.
//!
//! Modules:
//! - [`token`]: Authentication token structures and scopes.
//! - [`sub_protocol`]: Definitions for handshake, command, and data payloads.
//! - [`algorithms`]: Supported encryption and signature algorithms.
//! - [`tls_provider`]: Defines an encryption trait.
//! - [`tunnel`]: High level encrpted tunnel structs.
//!
//! See the documentation of each module for details.

mod macros;

pub mod algorithms;
pub mod sub_protocol;
pub mod tls_provider;
pub mod token;
pub mod tunnel;

#[macro_export]
macro_rules! random_bytes {
    ($n:expr) => {{
        use rand::rand_core::{OsRng, TryRngCore};

        let mut key = [0u8; $n];
        OsRng.try_fill_bytes(&mut key).unwrap();

        key
    }};
}
