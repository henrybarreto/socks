/// SOCKS response packet.
///
/// This structure represents a Response packet to wrote on TCP stream.
#[derive(Debug, Clone)]
pub struct Response {
    /// Version, null byte when in response.
    pub version: u8,
    /// Reply code.
    ///
    /// This is the only field that is really important. The SOCKS4 protocol specifies that the
    /// values of the others bytes should be ignored on the response.
    pub reply: u8,
    /// Reserved.
    pub rsv: u8,
    /// Destination IP, as above – the ip:port the client should bind to. Ignored on response.
    pub ip: [u8; 5],
    /// Destination port, meaningful if granted in BIND, otherwise ignore. Ignored on response.
    pub port: [u8; 2],
}

impl Response {
    pub fn new(reply: u8, ip: &[u8; 5], port: &[u8; 2]) -> Self {
        return Response {
            version: 0x05,
            reply: reply,
            rsv: 0x00,
            ip: [ip[0], ip[1], ip[2], ip[3], ip[4]],
            port: [port[0], port[1]],
        };
    }
}

const RESPONSE_REPLY_BUFFER_POSITION: usize = 1;

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        let mut buffer = [0 as u8; 8];

        buffer[RESPONSE_REPLY_BUFFER_POSITION] = self.reply;
        // The others fields are ignored due to RFC.

        return buffer.to_vec();
    }
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub version: u8,
    pub choose: u8,
}

impl From<Vec<u8>> for Choice {
    fn from(buffer: Vec<u8>) -> Self {
        return Choice {
            version: buffer[0],
            choose: buffer[1],
        };
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Ipv4 = 0x01,       // IPv4 address
    DomainName = 0x03, // Domain name
    Ipv6 = 0x04,       // IPv6 address
    Unknown = 0xFF,    // Unknown or unsupported address type
}

impl Type {
    // Returns a description of the address type
    pub fn description(&self) -> &str {
        match self {
            Type::Ipv4 => "IPv4 address",
            Type::DomainName => "Domain name",
            Type::Ipv6 => "IPv6 address",
            Type::Unknown => "Unknown address type",
        }
    }
}

// Implement `From<u8>` for infallible conversion from `u8` to `AddressType`
impl From<u8> for Type {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Type::Ipv4,
            0x03 => Type::DomainName,
            0x04 => Type::Ipv6,
            _ => Type::Unknown, // Handle unknown types
        }
    }
}

// Implement `Into<u8>` for infallible conversion from `AddressType` to `u8`
impl Into<u8> for Type {
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
