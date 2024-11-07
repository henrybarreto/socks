use super::Reply;

/// SOCKS5 response packet.
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
    pub ip: Vec<u8>,
    /// Destination port, meaningful if granted in BIND, otherwise ignore. Ignored on response.
    pub port: [u8; 2],
}

impl Response {
    pub fn new(reply: Reply, ip: Vec<u8>, port: [u8; 2]) -> Self {
        return Response {
            version: 0x05,
            reply: reply.into(),
            rsv: 0x00,
            ip,
            port,
        };
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        let mut buffer = vec![self.version, self.reply, self.rsv];
        buffer.append(&mut self.ip.clone());
        buffer.append(&mut self.port.clone().to_vec());

        return buffer.to_vec();
    }
}

#[derive(Debug, Clone)]
pub struct Choice {
    /// SOCKS version, should be 0x05 to represent SOCKS5.
    pub version: u8,
    /// Chosen authentication method.
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

impl Into<[u8; 2]> for Choice {
    fn into(self) -> [u8; 2] {
        let mut buffer = [0 as u8; 2];
        buffer[0] = self.version;
        buffer[1] = self.choose;

        return buffer;
    }
}

impl Default for Choice {
    fn default() -> Self {
        return Choice {
            version: 0x05,
            choose: 0x00,
        };
    }
}
