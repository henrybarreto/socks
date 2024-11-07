//! Common protocol functionality shared between SOCKS versions.
//!
//! This module provides abstractions for common operations like data relay,
//! connection establishment, and buffer management that are shared between
//! SOCKS4 and SOCKS5 implementations.

pub mod connection;
pub mod relay;

pub use connection::*;
pub use relay::*;
