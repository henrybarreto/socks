//! SOCKS request packet, and utilities.
use std::net::IpAddr;

/// Versions.
///
/// # Example
///
/// ```rust
/// let version: u8 = Versions::V4 as u8;
/// ```
#[derive(Debug, Clone)]
pub enum Versions {
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

/// Command code.
///
/// # Example
///
/// ```rust
/// let command: u8 = Commands::Connect as u8;
/// ```
#[derive(Debug, Clone)]
pub enum Commands {
    Invalid = 0x00,
    /// Establish a TCP/IP stream connection.
    Connect = 0x01,
    /// Establish a TCP/IP port binding.
    Bind = 0x02,
}

/// Min size of the Request packet received by SOCKS proxy server.
pub const SOCKS4_REQUEST_MIN_SIZE: usize = 9;

/// SOCKS request packet.
///
/// This structure represents a Request packet read from TCP stream;
#[derive(Debug, Clone)]
pub struct Request {
    /// Version number.
    pub version: u8,
    /// Command code.
    pub command: u8,
    /// Destination port number (in network byte order).
    pub port: [u8; 2],
    /// Destination IPv4 Address, 4 bytes (in network byte order).
    pub ip: [u8; 4],
    /// The user ID string, variable length, null-terminated.
    pub id: Vec<u8>,
}

impl Request {
    pub fn get_version(&self) -> Versions {
        return match self.version {
            0x04 => Versions::V4,
            0x05 => Versions::V5,
            _ => Versions::Invalid,
        };
    }

    pub fn get_command(&self) -> Commands {
        return match self.version {
            0x01 => Commands::Connect,
            0x02 => Commands::Bind,
            _ => Commands::Invalid,
        };
    }

    pub fn get_port(&self) -> u16 {
        return u16::from_be_bytes(self.port);
    }

    pub fn get_ip(&self) -> IpAddr {
        return IpAddr::from(self.ip);
    }
}

impl From<Vec<u8>> for Request {
    fn from(buffer: Vec<u8>) -> Self {
        return Request {
            version: buffer[0],
            command: buffer[1],
            port: [buffer[2], buffer[3]],
            ip: [buffer[4], buffer[5], buffer[6], buffer[7]],
            id: Vec::from(&buffer[7..]),
        };
    }
}
