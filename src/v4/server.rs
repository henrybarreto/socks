use super::Reply;

/// The size of the Response packet sent by SOCKS proxy server.
pub const SOCKS4_RESPONSE_SIZE: usize = 8;

/// SOCKS4 response packet.
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
