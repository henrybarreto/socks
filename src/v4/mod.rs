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
use std::io::Error;

use client::Request;
use server::Response;

pub mod client;
pub mod server;

pub trait Read {
    /// Reads a SOCKS request from a stream.
    fn read(
        stream: impl std::io::Read + std::io::Write,
        buffer: &mut [u8],
    ) -> Result<Request, Error>;
}

#[cfg(feature = "async")]
pub trait ReadAsync<T> {
    /// Reads a SOCKS request from a async stream.
    fn read_async(
        stream: &mut T,
        buffer: &mut [u8],
    ) -> impl std::future::Future<Output = Result<Request, Error>> + Send;
}

pub trait Write {
    /// Writes a SOCKS response to a stream.
    fn write(stream: impl std::io::Read + std::io::Write, response: Response) -> Result<(), Error>;
}

#[cfg(feature = "async")]
pub trait WriteAsync<T> {
    /// Writes a SOCKS response to a async stream.
    fn write_async(
        stream: &mut T,
        response: Response,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
