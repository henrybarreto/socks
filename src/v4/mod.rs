//! SOCKS v4 module.
//!
//! The `v4` module provides functionality for implementing the SOCKS4 protocol,
//! allowing clients to establish connections through a SOCKS4 proxy server.
//!
//! ## Overview
//!
//! SOCKS4 is a networking protocol that facilitates communication between a client
//! and a server through a proxy. It is primarily used for TCP connections and is
//! known for its simplicity and ease of use.

pub mod client;
pub mod server;
pub mod socks;

/// Reply code.
///
/// # Example
///
/// ```rust
/// use socks::v4::Reply;
///
/// let reply: u8 = Reply::Granted as u8;
/// ```
#[derive(Debug, Clone)]
pub enum Reply {
    /// Request granted.
    Granted = 0x5A,
    /// Request rejected or failed.
    RejectOrFailed = 0x5B,
    /// Request failed because client is not running identd (or not reachable from server).
    FailedClientNotRunning = 0x5C,
    /// Request failed because client's identd could not confirm the user ID in the request.
    FailedClientNotConfirmed = 0x5D,
}

impl Into<u8> for Reply {
    fn into(self) -> u8 {
        return self as u8;
    }
}
