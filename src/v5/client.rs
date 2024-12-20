use std::net::IpAddr;

use crate::{Command, Version};

/// SOCKS5 request packet.
#[derive(Debug, Clone)]
pub struct Request {
    /// Version number.
    pub version: u8,
    /// Command code.
    pub command: u8,
    /// Reserved.
    pub rsv: u8,
    /// Destination address with its type.
    pub addr: Vec<u8>,
    /// Destination port number.
    pub port: [u8; 2],
}

impl Request {
    pub fn get_version(&self) -> Version {
        return Version::from(self.version);
    }

    pub fn get_command(&self) -> Command {
        return Command::from(self.command);
    }

    pub fn get_addr(&self) -> IpAddr {
        let addr = Address::from(self.addr.clone());
        match Kind::from(addr.kind) {
            Kind::Ipv4 => {
                // TODO: There is a simple of doing this.
                return IpAddr::from([
                    addr.address[0],
                    addr.address[1],
                    addr.address[2],
                    addr.address[3],
                ]);
            }
            Kind::Ipv6 => {
                // TODO: There is a simple of doing this.
                return IpAddr::from([
                    addr.address[0],
                    addr.address[1],
                    addr.address[2],
                    addr.address[3],
                    addr.address[4],
                    addr.address[5],
                    addr.address[6],
                    addr.address[7],
                    addr.address[8],
                    addr.address[9],
                    addr.address[10],
                    addr.address[11],
                    addr.address[12],
                    addr.address[13],
                    addr.address[14],
                    addr.address[15],
                ]);
            }
            _ => {
                todo!();
            }
        }
    }

    pub fn get_port(&self) -> u16 {
        return u16::from_be_bytes(self.port);
    }
}

impl From<&[u8]> for Request {
    fn from(buffer: &[u8]) -> Self {
        let length = buffer.len();
        return Request {
            version: buffer[0],
            command: buffer[1],
            rsv: buffer[2],
            addr: Vec::from(&buffer[3..(length - 2)]),
            port: [buffer[length - 2], buffer[length - 1]],
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
    pub number: u8,
    pub auth: Vec<u8>,
}

impl From<&[u8]> for Greeting {
    fn from(buffer: &[u8]) -> Self {
        return Greeting {
            version: buffer[0],
            number: buffer[1],
            auth: Vec::from(&buffer[2..]),
        };
    }
}

#[derive(Debug, Clone)]
pub enum Kind {
    Ipv4 = 0x01,
    DomainName = 0x03,
    Ipv6 = 0x04,
    Unknown = 0xFF,
}

impl Kind {
    pub fn description(&self) -> &str {
        match self {
            Kind::Ipv4 => "IPv4 address",
            Kind::DomainName => "Domain name",
            Kind::Ipv6 => "IPv6 address",
            Kind::Unknown => "Unknown address type",
        }
    }
}

impl From<u8> for Kind {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Kind::Ipv4,
            0x03 => Kind::DomainName,
            0x04 => Kind::Ipv6,
            _ => Kind::Unknown, // Handle unknown types
        }
    }
}

impl Into<u8> for Kind {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    pub kind: u8,
    pub address: Vec<u8>,
}

impl From<Vec<u8>> for Address {
    fn from(buffer: Vec<u8>) -> Self {
        return Address {
            kind: buffer[0],
            address: Vec::from(&buffer[1..]),
        };
    }
}
