/*!
SOCKS proxy library.

# Overview

SOCKS proxy library provides a comprehensive suite of functions, methods, and
structures for building and managing SOCKS proxy servers. The idea is to allow
developers to easily create, configure, and deploy SOCKS proxies for secure,
anonymous internet communication.

# Example

```rust
use std::net::{SocketAddr, TcpListener, TcpStream};

use socks::{
    request::Request,
    response::{Reply, Response},
    v4::SocksStream,
    Read as SocksRead, Write as SocksWrite,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1080").unwrap();
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(mut stream) => {
                let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                let request: Request = match SocksStream::read(&mut stream, &mut buffer) {
                    Ok(request) => request,
                    Err(e) => {
                        dbg!(e);

                        return;
                    }
                };

                dbg!(&request);

                let ip = request.get_ip();
                let port = request.get_port();

                dbg!(ip);
                dbg!(port);

                let addr: SocketAddr = SocketAddr::new(ip, port);
                let mut connection = TcpStream::connect(addr).unwrap();

                let response = Response::new(Reply::Granted);
                dbg!(&response);

                SocksStream::write(&mut stream, response).unwrap();

                // Pipe data between the socket and connection.
            }
            Err(e) => {
                dbg!(e);

                return;
            }
        }
    }
}
```

## References
- [RFC 1928 - SOCKS Protocol Specification](https://datatracker.ietf.org/doc/html/rfc1928)
- [Wikipedia - SOCKS](https://en.wikipedia.org/wiki/SOCKS)
*/

use std::io::Error;

pub mod request;
pub mod response;
pub mod v4;

use request::Request;
use response::Response;

pub trait Read {
    /// Reads a SOCKS request from a stream.
    fn read(
        stream: impl std::io::Read + std::io::Write,
        buffer: &mut [u8],
    ) -> Result<Request, Error>;
}

#[cfg(feature = "tokio")]
pub trait ReadTokio {
    /// Reads a SOCKS request from a tokion stream.
    fn read_async(
        stream: &mut tokio::net::TcpStream,
        buffer: &mut [u8],
    ) -> impl std::future::Future<Output = Result<Request, Error>> + Send;
}

pub trait Write {
    /// Writes a SOCKS response to a stream.
    fn write(stream: impl std::io::Read + std::io::Write, response: Response) -> Result<(), Error>;
}

#[cfg(feature = "tokio")]
pub trait WriteTokio {
    /// Writes a SOCKS response to a tokio stream;
    fn write_async(
        stream: &mut tokio::net::TcpStream,
        response: Response,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
