//! SOCKS v5 module.
//!
//! The `v5` module provides functionality for implementing the SOCKS5 protocol,
//! allowing clients to establish connections through a SOCKS5 proxy server.
//!
//! ## Overview
//!
//! It is an incompatible extension of the SOCKS4 protocol; it offers more choices for
//! authentication and adds support for IPv6 and UDP, the latter of which can be used for DNS
//! lookups.

pub mod client;
pub mod server;
pub mod socks;

/// Reply code.
///
/// # Example
///
/// ```rust
/// use socks::v5::Reply;
///
/// let reply: u8 = Reply::RequestGranted as u8;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Reply {
    RequestGranted = 0x00,
    GeneralFailure = 0x01,
    ConnectionNotAllowedByRuleset = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefusedByDestinationHost = 0x05,
    TtlExpired = 0x06,
    CommandNotSupportedOrProtocolError = 0x07,
    AddressTypeNotSupported = 0x08,
}

impl From<u8> for Reply {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Reply::RequestGranted,
            0x01 => Reply::GeneralFailure,
            0x02 => Reply::ConnectionNotAllowedByRuleset,
            0x03 => Reply::NetworkUnreachable,
            0x04 => Reply::HostUnreachable,
            0x05 => Reply::ConnectionRefusedByDestinationHost,
            0x06 => Reply::TtlExpired,
            0x07 => Reply::CommandNotSupportedOrProtocolError,
            0x08 => Reply::AddressTypeNotSupported,
            _ => panic!("Unknown error code: {}", byte),
        }
    }
}

impl Into<u8> for Reply {
    fn into(self) -> u8 {
        self as u8
    }
}
