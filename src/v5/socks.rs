use std::{io::Error, net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use tracing::{debug, error, info, span, trace, warn, Instrument, Level};

use crate::{
    common::{relay, Connection},
    v5::{
        client::{Greeting, Request},
        server::Response,
        Reply,
    },
    Command,
};

use super::server::Choice;

pub trait Handler: Send + Sync + 'static {
    fn auth(&self, greeting: Greeting) -> Result<Choice, Error>;
    fn request(&self, request: Request) -> Result<Reply, Error>;
}

pub struct Socks {
    handler: Arc<dyn Handler>,
}

impl Socks {
    pub fn new(internal: impl Handler) -> Self {
        info!("initializing server with custom handler");
        return Socks {
            handler: Arc::new(internal),
        };
    }

    pub async fn listen(&self, addr: impl ToSocketAddrs) -> Result<(), Error> {
        let listener = TcpListener::bind(addr).await?;
        let local_addr = listener.local_addr()?;
        info!(local_addr = %local_addr, "server listening for connections");

        loop {
            let (stream, _) = listener.accept().await?;
            let peer_addr = match stream.peer_addr() {
                Ok(addr) => addr,
                Err(e) => {
                    error!(error = %e, "failed to get peer address for incoming connection");
                    continue;
                }
            };
            info!(peer_addr = %peer_addr, "new client connection accepted");

            let handler = Arc::clone(&self.handler);
            let peer_addr_clone = peer_addr.clone();

            task::spawn(
                async move {
                    info!("spawned new handler task");
                    trace!("processing new TCP stream");

                    let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                    // Greeting phase
                    let mut connection = Connection::new(stream);

                    debug!("reading greeting from client");
                    let greeting = match connection.read_greeting(&mut buffer).await {
                        Ok(g) => {
                            info!("received greeting from client");
                            g
                        }
                        Err(e) => {
                            error!(error = %e, "failed to read greeting from client");

                            return;
                        }
                    };


                    // Authentication phase
                    debug!("processing authentication request");
                    let choice = match handler.auth(greeting.clone()) {
                        Ok(c) => {
                            info!(auth_method = ?c, "authentication successful");
                            c
                        }
                        Err(e) => {
                            error!(error = %e, "authentication failed");
                            return;
                        }
                    };

                    if let Err(e) = connection.write_choice(choice).await {
                        error!(error = ?e, "error writing authentication choice to stream");

                        return;
                    }

                    let request = match connection.read_request::<Request>(buffer).await {
                        Ok(r) => {
                            info!(?r, "received request from client");
                            r
                        }
                        Err(e) => {
                            error!(error = %e, "failed to read request from client");
                            return;
                        }
                    };

                    info!(?request, "received request from client");

                    let ip = if let Some(addr) = request.get_addr() {
                        addr
                    } else {
                        error!("failed to resolve address from request");

                        return;
                    };

                    let port = request.get_port();
                    let command = request.get_command();
                    let target_addr: SocketAddr = SocketAddr::new(ip, port);

                    info!("processing request");

                    async {
                        match command {
                            Command::Connect => {
                                info!("establishing connection to target");
                                // TODO: Add timeout for connection.
                                let target = match TcpStream::connect(target_addr).await {
                                    Ok(t) => {
                                        info!("successfully connected to target");
                                        t
                                    }
                                    Err(e) => {
                                        error!(error = %e, "failed to connect to target");

                                        if let Err(e) = connection
                                            .write_response(Response::new(
                                                Reply::GeneralFailure,
                                                request.addr.to_vec(),
                                                request.port.clone(),
                                            ))
                                            .await
                                        {
                                            error!(error = ?e, "error writing connection failure response to stream");
                                        }

                                        warn!("connection to target failed, sent failure response");

                                        return;
                                    }
                                };

                                info!("processing request through handler");
                                let reply = match handler.request(request.clone()) {
                                    Ok(r) => {
                                        info!(reply = ?r, "handler approved request");
                                        r
                                    }
                                    Err(e) => {
                                        error!(error = ?e, "handler rejected request");

                                        if let Err(e) = connection
                                            .write_response(Response::new(
                                                Reply::GeneralFailure,
                                                request.addr.to_vec(),
                                                request.port.clone(),
                                            ))
                                            .await
                                        {
                                            error!(error = ?e, "error writing handler failure response");
                                        }

                                        return;
                                    }
                                };

                                let response = Response::new(
                                    reply.clone(),
                                    request.addr.to_vec(),
                                    request.port.clone(),
                                );

                                if let Err(e) = connection.write_response(response).await {
                                    error!(error = ?e, "error writing success response to stream");

                                    return;
                                }

                                info!("starting data relay between client and target");

                                let stats =
                                    relay::relay_data(connection.into(), target).await;

                                info!(
                                    stats.bytes_to_client,
                                    stats.bytes_to_target,
                                    stats.packets_to_client,
                                    stats.packets_to_target,
                                    "relay completed"
                                );
                            }
                            _ => {
                                info!(?command, "command is not supported");
                            }
                        }
                    }
                    .instrument(
                        span!(Level::INFO, "target", target_addr = %target_addr, command = ?command),
                    )
                    .await;

                    info!("handler completed");
                }
                .instrument(span!(Level::INFO, "socks5", peer_addr = %peer_addr_clone)),
            );
        }
    }
}
