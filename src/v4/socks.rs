use std::{io::Error, net::SocketAddr};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::{
    v4::{client::Request, server::Response},
    Command,
};

use super::Reply;

pub struct Socks;

impl Socks {
    pub fn new() -> Self {
        return Socks;
    }

    pub async fn listen(
        &self,
        addr: impl ToSocketAddrs,
        handler: fn(addr: SocketAddr) -> Reply,
    ) -> Result<(), Error> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            println!("New TCP stream accepted");

            tokio::spawn(async move {
                println!("new tcp stream");

                let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                let read = stream.read(&mut buffer).await;
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

                let ip = request.get_addr();
                let port = request.get_port();
                let command = request.get_command();

                let addr: SocketAddr = SocketAddr::new(ip, port);

                match command {
                    Command::Connect => {
                        println!("Connecting to {}:{}", ip, port);

                        let mut connection = match TcpStream::connect(addr).await {
                            Ok(connection) => connection,
                            Err(e) => {
                                dbg!(&e);

                                let response = Response::new(Reply::RejectOrFailed);
                                let response_buffer: Vec<u8> = response.into();

                                let wrote = stream.write(&response_buffer).await;
                                if let Err(e) = wrote {
                                    dbg!(e);

                                    return;
                                }

                                println!("failed to connect to ending host");

                                return;
                            }
                        };

                        let reply = handler(addr);

                        let response = Response::new(reply.clone());
                        let response_buffer: Vec<u8> = response.into();

                        let wrote = stream.write(&response_buffer).await;
                        if let Err(e) = wrote {
                            dbg!(e);

                            return;
                        }

                        let size = wrote.unwrap();
                        if size == 0 {
                            return;
                        }

                        if let Reply::Granted = reply {
                            println!("Access allowed");
                        } else {
                            eprintln!("Access not allowed");

                            return;
                        }

                        drop(buffer);

                        let (mut connection_read, mut connection_write) = connection.split();
                        let (mut stream_read, mut stream_write) = stream.split();

                        let mut buffer_connection = vec![0u8; 65535];
                        let mut buffer_stream = vec![0u8; 65535];

                        loop {
                            tokio::select! {
                                Ok(size) = connection_read.read(&mut buffer_connection) => {
                                    if size == 0 {
                                        break;
                                    }
                                    if let Err(e) = stream_write.write_all(&buffer_connection[..size]).await {
                                        eprintln!("Error writing to stream: {:?}", e);
                                        break;
                                    }
                                },
                                Ok(size) = stream_read.read(&mut buffer_stream) => {
                                    if size == 0 {
                                        break;
                                    }
                                    if let Err(e) = connection_write.write_all(&buffer_stream[..size]).await {
                                        eprintln!("Error writing to connection: {:?}", e);
                                        break;
                                    }
                                },
                            };
                        }
                    }
                    _ => {
                        todo!();
                    }
                }
            });
        }
    }
}
