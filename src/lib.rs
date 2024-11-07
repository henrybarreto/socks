/*!
SOCKS proxy library.

# Overview

SOCKS proxy library provides a comprehensive suite of functions, methods, and
structures for building and managing SOCKS proxy servers. The idea is to allow
developers to easily create, configure, and deploy SOCKS proxies for secure,
anonymous internet communication.

## References
- [RFC 1928 - SOCKS Protocol Specification](https://datatracker.ietf.org/doc/html/rfc1928)
- [Wikipedia - SOCKS](https://en.wikipedia.org/wiki/SOCKS)
*/

pub mod common;
pub mod v4;
pub mod v5;

/// Versions.
///
/// # Example
///
/// ```rust
/// use socks::Version;
///
/// let version: u8 = Version::V4 as u8;
/// ```
#[derive(Debug, Clone)]
pub enum Version {
    Invalid = 0x00,
    V4 = 0x04,
    /**
    The SOCKS5 protocol is defined in RFC 1928. It is an incompatible extension of the SOCKS4
    protocol; it offers more choices for authentication and adds support for IPv6 and UDP, the
    latter of which can be used for DNS lookups.

    <https://datatracker.ietf.org/doc/html/rfc1928>
    */
    V5 = 0x05,
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            4 => Self::V4,
            5 => Self::V5,
            _ => Self::Invalid,
        }
    }
}

/// Command code.
///
/// # Example
///
/// ```rust
/// use socks::Command;
///
/// let command: u8 = Command::Connect as u8;
/// ```
#[derive(Debug, Clone)]
pub enum Command {
    Invalid = 0x00,
    /// Establish a TCP stream connection.
    Connect = 0x01,
    /// Establish a TCP port binding.
    Bind = 0x02,
    Associate = 0x03,
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        return match value {
            0x01 => Command::Connect,
            0x02 => Command::Bind,
            0x03 => Command::Associate,
            _ => Self::Invalid,
        };
    }
}
