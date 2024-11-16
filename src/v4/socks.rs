use std::{
    io::{Error, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    thread,
};

use crate::{
    v4::{client::Request, server::Response},
    Command, Version,
};

use super::Reply;

pub struct Socks {}

impl Socks {
    pub fn new() -> Self {
        return Socks {};
    }

    pub fn listen(
        &self,
        addr: impl ToSocketAddrs,
        handler: fn(addr: SocketAddr) -> Reply,
    ) -> Result<(), Error> {
        // NOTE: Implementation for tests only.
        let listener = TcpListener::bind(addr).unwrap();
        for tcp_stream in listener.incoming() {
            match tcp_stream {
                Ok(mut stream) => {
                    println!("New TCP stream accepted");

                    thread::spawn(move || {
                        println!("new tcp stream");

                        let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                        let read = stream.read(&mut buffer);
                        if let Err(e) = read {
                            dbg!(e);

                            return;
                        }

                        let size = read.unwrap();

                        if size == 0 {
                            return;
                        }

                        let request = Request::from(&buffer[..size]);

                        println!("Received SOCKS4 request: {:?}", request);

                        let ip = request.get_ip();
                        let port = request.get_port();
                        let addr: SocketAddr = SocketAddr::new(ip, port);

                        match Command::from(request.command) {
                            Command::Connect => {
                                println!("Connecting to {}:{}", ip, port);

                                let mut connection = match TcpStream::connect(addr) {
                                    Ok(connection) => connection,
                                    Err(e) => {
                                        eprintln!("Failed to connect to destination: {:?}", e);

                                        return;
                                    }
                                };

                                let reply = handler(addr);
                                let response = Response::new(reply.clone());
                                let response_buffer: Vec<u8> = response.into();

                                let wrote = stream.write(&response_buffer);
                                if let Err(e) = wrote {
                                    dbg!(e);

                                    return;
                                }

                                let size = wrote.unwrap();
                                dbg!(size);

                                if let Reply::Granted = reply {
                                    println!("Access allowed");
                                } else {
                                    eprintln!("Access not allowed");

                                    return;
                                }

                                let mut s = stream.try_clone().unwrap();
                                let mut c = connection.try_clone().unwrap();
                                let _ = thread::spawn(move || {
                                    let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                                    loop {
                                        match c.read(&mut buffer) {
                                            Ok(0) => {
                                                break;
                                            }
                                            Ok(size) => {
                                                if let Err(e) = s.write_all(&buffer[..size]) {
                                                    eprintln!("Error writing to stream: {:?}", e);

                                                    break;
                                                }
                                            }
                                            Err(e) => {
                                                dbg!(e);

                                                break;
                                            }
                                        }
                                    }
                                });

                                let mut buffer: Vec<u8> = vec![0 as u8; 65535];
                                loop {
                                    match stream.read(&mut buffer) {
                                        Ok(0) => {
                                            break;
                                        }
                                        Ok(size) => {
                                            if let Err(e) = connection.write_all(&buffer[..size]) {
                                                eprintln!("Error writing to connection: {:?}", e);

                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            dbg!(e);

                                            break;
                                        }
                                    }
                                }
                            }
                            _ => {
                                eprintln!("SOCKS doesn't support this command yet");

                                return;
                            }
                        }

                        println!("SOCKS4 connection closed");
                    });
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }

        return Ok(());
    }
}
