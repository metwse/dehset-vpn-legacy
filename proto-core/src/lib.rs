//! Core protocol definitions and shared types for the VPN system.
//!
//! Low-level message structures, token formats, and sub-protocols used across
//! all VPN-related components are defined in this crate.
//!
//! Modules:
//! - [`token`]: Authentication token structures and scopes.
//! - [`sub_protocol`]: Definitions for handshake, command, and data payloads.
//!
//! See the documentation of each module for details.

pub mod sub_protocol;
pub mod token;

pub use sub_protocol::*;
pub use token::*;
