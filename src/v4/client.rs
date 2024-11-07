//! SOCKS request packet, and utilities.

use std::net::IpAddr;

use crate::{Command, Version};

/// SOCKS4 request packet.
#[derive(Debug, Clone)]
pub struct Request {
    /// Version number.
    pub version: u8,
    /// Command code.
    pub command: u8,
    /// Destination port number (in network byte order).
    pub port: [u8; 2],
    /// Destination IPv4 Address, 4 bytes (in network byte order).
    pub addr: [u8; 4],
    /// The user ID string, variable length, null-terminated.
    pub id: String,
}

impl Request {
    pub fn get_version(&self) -> Version {
        return Version::from(self.version);
    }

    pub fn get_command(&self) -> Command {
        return Command::from(self.command);
    }

    pub fn get_port(&self) -> u16 {
        return u16::from_be_bytes(self.port);
    }

    pub fn get_addr(&self) -> IpAddr {
        return IpAddr::from(self.addr);
    }
}

impl From<&[u8]> for Request {
    fn from(buffer: &[u8]) -> Self {
        // TODO: Avoid panics by checking the length of the buffer.
        return Request {
            version: buffer[0],
            command: buffer[1],
            port: [buffer[2], buffer[3]],
            addr: [buffer[4], buffer[5], buffer[6], buffer[7]],
            id: String::from_utf8_lossy(&buffer[7..]).to_string(),
        };
    }
}

impl From<Vec<u8>> for Request {
    fn from(buffer: Vec<u8>) -> Self {
        return Request::from(&buffer[..]);
    }
}
