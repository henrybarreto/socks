//! SOCKS reponse packet, and utilities.

/// Peply code.
///
/// # Example
///
/// ```rust
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

/// The size of the Response packet sent by SOCKS proxy server.
pub const SOCKS4_RESPONSE_SIZE: usize = 8;

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
    /// Destination port, meaningful if granted in BIND, otherwise ignore. Ignored on response.
    pub port: [u8; 2],
    /// Destination IP, as above – the ip:port the client should bind to. Ignored on response.
    pub ip: [u8; 4],
}

impl Response {
    pub fn new(reply: Reply) -> Self {
        // NOTE: The field `reply`  is the only field that is really important. The SOCKS4 protocol specifies that the
        // values of the others bytes should be ignored on the response.
        return Response {
            version: 0x00,
            reply: reply.into(),
            port: [0x00, 0x00],
            ip: [0x00, 0x00, 0x00, 0x00],
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
