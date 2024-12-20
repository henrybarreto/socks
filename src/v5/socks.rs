use std::{future::Future, io::Error, net::SocketAddr};

use log::{debug, error, info, trace, warn};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    select, task,
};

use crate::{
    v5::{
        client::{Greeting, Request},
        server::{self},
        Reply,
    },
    Command,
};

pub struct Socks;

impl Socks {
    pub fn new() -> Self {
        return Socks;
    }

    pub async fn listen<A, H, FutA, FutH>(
        &self,
        addr: impl ToSocketAddrs,
        auth: A,
        handler: H,
    ) -> Result<(), Error>
    where
        A: Fn(TcpStream, Greeting) -> FutA + Send + Copy + 'static,
        H: Fn(TcpStream, Request) -> FutH + Send + Copy + 'static,
        FutA: Future<Output = Result<TcpStream, Error>> + Send + 'static,
        FutH: Future<Output = Result<TcpStream, Error>> + Send + 'static,
    {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            info!("new stream accepted from: {}", &stream.peer_addr().unwrap());

            task::spawn(async move {
                trace!("new tcp stream");

                let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                let read = stream.read(&mut buffer).await;
                if let Err(e) = read {
                    error!("error on greating read from stream: {:?}", e);

                    return;
                }

                let size = read.unwrap();
                if size == 0 {
                    error!("nothing was read from on greeting request");

                    return;
                }

                let greeting = Greeting::from(&buffer[..size]);

                let mut stream = match auth(stream, greeting).await {
                    Ok(s) => s,
                    Err(e) => {
                        error!("failed to authenticate: {}", e);

                        return;
                    }
                };

                let read = stream.read(&mut buffer).await;
                if let Err(e) = read {
                    error!("error on read the request from stream: {:?}", e);

                    return;
                }

                let size = read.unwrap();
                if size == 0 {
                    error!("nothing was read from request");

                    return;
                }

                let request = Request::from(&buffer[..size]);
                drop(buffer);

                debug!("received SOCKS5 request: {:?}", request);

                let ip = request.get_addr();
                let port = request.get_port();
                let command = request.get_command();

                let addr: SocketAddr = SocketAddr::new(ip, port);

                match command {
                    Command::Connect => {
                        info!("trying to connect to {}:{}", ip, port);

                        let mut connection = match TcpStream::connect(addr).await {
                            Ok(connection) => connection,
                            Err(e) => {
                                dbg!(e);

                                let response = server::Response::new(
                                    Reply::GeneralFailure,
                                    request.addr.to_vec(),
                                    request.port,
                                );
                                let response_buffer: Vec<u8> = response.into();

                                let written = stream.write(&response_buffer).await;
                                if let Err(e) = written {
                                    error!("error on response written to stream: {:?}", e);

                                    return;
                                }

                                trace!("failed to connect to ending host");

                                return;
                            }
                        };

                        let mut stream = handler(stream, request).await.unwrap();

                        let (mut connection_read, mut connection_write) = connection.split();
                        let (mut stream_read, mut stream_write) = stream.split();

                        let mut buffer_connection = vec![0u8; 65535];
                        let mut buffer_stream = vec![0u8; 65535];

                        loop {
                            select! {
                                Ok(size) = connection_read.read(&mut buffer_connection) => {
                                    if size == 0 {
                                        break;
                                    }
                                    if let Err(e) = stream_write.write(&buffer_connection[..size]).await {
                                        error!("error writing to stream on loop: {:?}", e);

                                        break;
                                    }
                                },
                                Ok(size) = stream_read.read(&mut buffer_stream) => {
                                    if size == 0 {
                                        break;
                                    }
                                    if let Err(e) = connection_write.write(&buffer_stream[..size]).await {
                                        error!("error writing to connection on loop: {:?}", e);

                                        break;
                                    }
                                },
                                else => {
                                    warn!("something was wrong on reading and writing process");

                                    break;
                                },
                            };
                        }

                        info!(
                            "stream from {} to {} done",
                            &stream.peer_addr().unwrap(),
                            &connection.peer_addr().unwrap(),
                        );
                    }
                    _ => {
                        todo!();
                    }
                }
            });
        }
    }
}
