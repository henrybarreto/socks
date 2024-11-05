//! SOCKS v4 module.
//!
//! The `v4` module provides functionality for implementing the SOCKS4 protocol,
//! allowing clients to establish connections through a SOCKS4 proxy server.
//!
//! ## Overview
//!
//! SOCKS4 is a networking protocol that facilitates communication between a client
//! and a server through a proxy. It is primarily used for TCP connections and is
//! known for its simplicity and ease of use.
//!
//! ## Features
//!
//! - Functions, methods, and structures to handle SOCKS4 requests.
//! - Usage of fundamental traits for IO as std::io::Read and std::io::Write.
//! - Async support.
use std::io::{Error, ErrorKind};

use crate::{request, response, Read, ReadTokio, Write, WriteTokio};

use request::Request;
use response::Response;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct SocksStream;

impl Read for SocksStream {
    fn read(
        mut stream: impl std::io::Read + std::io::Write,
        buffer: &mut [u8],
    ) -> Result<Request, Error> {
        let read = stream.read(buffer);
        if let Err(e) = read {
            return Err(e);
        }

        let size = read.unwrap();

        if size == 0 {
            return Err(Error::new(ErrorKind::Other, "stream was closed"));
        }

        if size < request::SOCKS4_REQUEST_MIN_SIZE {
            return Err(Error::new(
                ErrorKind::Other,
                "SOCKS inital request should have at least 8 bytes",
            ));
        }

        return Ok(Request::from(buffer[..size].to_vec()));
    }
}

#[cfg(feature = "tokio")]
impl ReadTokio for SocksStream {
    async fn read_async(
        stream: &mut tokio::net::TcpStream,
        buffer: &mut [u8],
    ) -> Result<Request, Error> {
        let read = stream.read(buffer).await;
        if let Err(e) = read {
            return Err(e);
        }

        let size = read.unwrap();

        if size == 0 {
            return Err(Error::new(ErrorKind::Other, "stream was closed"));
        }

        if size < request::SOCKS4_REQUEST_MIN_SIZE {
            return Err(Error::new(
                ErrorKind::Other,
                "SOCKS inital request should have at least 8 bytes",
            ));
        }

        return Ok(Request::from(buffer[..size].to_vec()));
    }
}

impl Write for SocksStream {
    fn write(
        mut stream: impl std::io::Read + std::io::Write,
        response: Response,
    ) -> Result<(), Error> {
        let response_buffer: Vec<u8> = response.into();

        let wrote = stream.write(&response_buffer);
        if let Err(e) = wrote {
            return Err(e);
        }

        let size = wrote.unwrap();
        if size != response::SOCKS4_RESPONSE_SIZE {
            return Err(Error::new(
                ErrorKind::Other,
                "SOCKS wrtie should write 8 bytes, but failed on it",
            ));
        }

        return Ok(());
    }
}

#[cfg(feature = "tokio")]
impl WriteTokio for SocksStream {
    async fn write_async(
        stream: &mut tokio::net::TcpStream,
        response: Response,
    ) -> Result<(), Error> {
        let response_buffer: Vec<u8> = response.into();

        let wrote = stream.write(&response_buffer).await;
        if let Err(e) = wrote {
            return Err(e);
        }

        let size = wrote.unwrap();
        if size != response::SOCKS4_RESPONSE_SIZE {
            return Err(Error::new(
                ErrorKind::Other,
                "SOCKS wrtie should write 8 bytes, but failed on it",
            ));
        }

        return Ok(());
    }
}
