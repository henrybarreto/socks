//! SOCKS request packet, and utilities.

use std::net::IpAddr;

use crate::{Command, Version};

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
    pub fn get_version(&self) -> Version {
        return Version::from(self.version);
    }

    pub fn get_command(&self) -> Command {
        return Command::from(self.command);
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
