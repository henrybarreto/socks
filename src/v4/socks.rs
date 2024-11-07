use std::{io::Error, net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use tracing::{debug, error, info, span, trace, warn, Instrument, Level};

use crate::{
    common::{relay, Connection},
    v4::{client::Request, server::Response},
    Command,
};

use super::Reply;

pub trait Handler: Send + Sync + 'static {
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

            task::spawn(async move {
                debug!("spawning new handler task");
                trace!("processing new TCP stream");

                let buffer: Vec<u8> = vec![0 as u8; 65535];

                let mut connection = Connection::new(stream);

                // Request phase
                let request = match connection.read_request::<Request>(buffer).await {
                    Ok(r) => {
                        debug!(?r, "received request from client");
                        r
                    },
                    Err(e) => {
                        error!(error = ?e, "failed to read request from client");
                        return;
                    }
                };

                debug!(?request, "received request from client");

                let ip = request.get_addr();
                let port = request.get_port();
                let command = request.get_command();
                let target_addr: SocketAddr = SocketAddr::new(ip, port);

                info!(command = ?command, "processing request");

                async {
                    match command {
                        Command::Connect => {
                            info!("establishing connection to target");
                            // TODO: Add timeout for connection.
                            let target = match TcpStream::connect(target_addr).await {
                                Ok(t) => {
                                    info!("successfully connected to target");
                                    t
                                },
                                Err(e) => {
                                    error!(error = %e, "failed to connect to target");

                                    if let Err(e) = connection
                                        .write_response(Response::new( Reply::RejectOrFailed))
                                        .await
                                    {
                                        error!(error = ?e, "error writing connection failure response to stream");
                                    }

                                    warn!("connection to target failed, sent failure response");
                                    return;
                                }
                            };

                            debug!("processing request through handler");
                            let reply = match handler.request(request.clone()) {
                                Ok(r) => {
                                    info!(reply = ?r, "handler approved request");
                                    r
                                },
                                Err(e) => {
                                    error!(error = ?e, "handler rejected request");

                                    if let Err(e) = connection
                                        .write_response(Response::new(Reply::RejectOrFailed))
                                        .await
                                    {
                                        error!(error = ?e, "error writing connection failure response to stream");
                                    }

                                    return;
                                }
                            };

                            let response = Response::new(reply.clone());

                            if let Err(e) = connection.write_response(response).await {
                                error!(error = ?e, "error writing success response");
                                return;
                            }

                            info!("starting data relay between client and target");

                            let stats = relay::relay_data(connection.into(), target).await;

                            info!(
                                stats.bytes_to_client,
                                stats.bytes_to_target,
                                stats.packets_to_client,
                                stats.packets_to_target,
                                "relay completed"
                            );
                        }
                        _ => {
                            info!(?command, "unsupported command");
                        }
                    }
                }.instrument(span!(Level::INFO, "target", command = ?command)).await;

                info!("handler completed");
            }.instrument(span!(Level::INFO,"socks4", peer_addr = %peer_addr_clone)));
        }
    }
}
