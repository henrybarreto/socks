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
    /// Reserved.
    pub rsv: u8,
    /// Destination IPv4 Address, 4 bytes (in network byte order).
    pub ip: [u8; 4],
    /// Destination port number (in network byte order).
    pub port: [u8; 2],
}

impl Request {
    pub fn get_version(&self) -> Version {
        return Version::from(self.version);
    }

    pub fn get_command(&self) -> Command {
        return Command::from(self.command);
    }

    pub fn get_ip(&self) -> IpAddr {
        return IpAddr::from(self.ip);
    }

    pub fn get_port(&self) -> u16 {
        return u16::from_be_bytes(self.port);
    }
}

impl From<Vec<u8>> for Request {
    fn from(buffer: Vec<u8>) -> Self {
        return Request {
            version: buffer[0],
            command: buffer[1],
            rsv: buffer[2],
            ip: [buffer[4], buffer[5], buffer[6], buffer[7]],
            port: [buffer[8], buffer[9]],
        };
    }
}

#[derive(Debug, Clone)]
pub enum AuthMethod {
    NoAuthentication = 0x00,
    Gssapi = 0x01,
    UsernamePassword = 0x02,
    Chapp = 0x03, // Challenge-Handshake Authentication Protocol
    Unassigned04 = 0x04,
    ChallengeResponse = 0x05,
    Ssl = 0x06, // Secure Sockets Layer
    NdsAuthentication = 0x07,
    MultiAuthenticationFramework = 0x08,
    JsonParameterBlock = 0x09,
    Unknown = 0xFF, // Optionally for any unsupported values
}

impl From<u8> for AuthMethod {
    fn from(value: u8) -> Self {
        return match value {
            0x00 => AuthMethod::NoAuthentication,
            0x01 => AuthMethod::Gssapi,
            0x02 => AuthMethod::UsernamePassword,
            0x03 => AuthMethod::Chapp,
            0x04 => AuthMethod::Unassigned04,
            0x05 => AuthMethod::ChallengeResponse,
            0x06 => AuthMethod::Ssl,
            0x07 => AuthMethod::NdsAuthentication,
            0x08 => AuthMethod::MultiAuthenticationFramework,
            0x09 => AuthMethod::JsonParameterBlock,
            _ => AuthMethod::Unknown,
        };
    }
}

impl Into<u8> for AuthMethod {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone)]
pub struct Greeting {
    pub version: u8,
    pub auth_method: u8,
    pub auth: Vec<u8>,
}

impl From<Vec<u8>> for Greeting {
    fn from(buffer: Vec<u8>) -> Self {
        return Greeting {
            version: buffer[0],
            auth_method: buffer[1],
            auth: Vec::from(&buffer[2..]),
        };
    }
}
