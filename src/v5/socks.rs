use std::{
    io::{Error, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    thread,
};

use crate::v5::{
    client::{Greeting, Request},
    server::{self, Choice},
    Reply,
};

pub struct Socks {}

impl Socks {
    pub fn new() -> Self {
        return Socks {};
    }

    pub fn listen(
        &self,
        addr: impl ToSocketAddrs,
        auth: fn(greeting: Greeting) -> Choice,
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

                        let read = stream.read(&mut buffer).unwrap();
                        let greeting = Greeting::from(&buffer[..read]);
                        dbg!(&greeting);

                        let choice = auth(greeting);

                        let choice_buffer: [u8; 2] = choice.into();
                        dbg!(&choice_buffer);

                        let written = stream.write(&choice_buffer).unwrap();
                        dbg!(written);

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

                        println!("Received SOCKS5 request: {:?}", request);

                        let ip = request.get_addr();
                        let port = request.get_port();
                        let addr: SocketAddr = SocketAddr::new(ip, port);

                        let reply = handler(addr);
                        let response = server::Response::new(
                            reply.clone(),
                            request.addr.to_vec(),
                            request.port,
                        );

                        let response_buffer: Vec<u8> = response.into();
                        dbg!(&response_buffer);

                        if let Err(e) = stream.write(&response_buffer) {
                            eprintln!("Error sending response: {:?}", e);

                            return;
                        }

                        if let Reply::RequestGranted = reply {
                            println!("Access allowed");
                        } else {
                            eprintln!("Access not allowed");

                            return;
                        }

                        println!("Connecting to {}:{}", ip, port);

                        let mut connection = match TcpStream::connect(addr) {
                            Ok(connection) => connection,
                            Err(e) => {
                                eprintln!("Failed to connect to destination: {:?}", e);

                                return;
                            }
                        };

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

                        println!("SOCKS5 connection closed");
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
