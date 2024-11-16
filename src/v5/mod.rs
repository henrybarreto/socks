pub mod client;
pub mod server;

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
    /// Establish a TCP stream connection.
    Connect = 0x01,
    /// Establish a TCP port binding.
    Bind = 0x02,
    /// Establish a UDP port.
    Associate = 0x03,
}

/// Reply code.
///
/// # Example
///
/// ```rust
/// let reply: u8 = Reply::Granted as u8;
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

// Implementing `From<u8>` for converting from a raw byte (u8) to the enum variant
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

// Implementing `Into<u8>` for converting the enum variant to a raw byte (u8)
impl Into<u8> for Reply {
    fn into(self) -> u8 {
        self as u8
    }
}
