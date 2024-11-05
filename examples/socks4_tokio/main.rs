/*!
This is a simple SOCKS4 server example using Tokio.

# Usage

## HTTPie

You can use HTTPie with `--proxy` flag to do an HTTP request through the SOCKS server.

```bash
http --proxy=http:socks4://localhost:1080 http://example.com
```

## Chromium

You can also test the server through a Chromium based browser.

```bash
chromium --proxy-server="socks4://localhost:1080"
```
*/
use socks::{
    response::{Reply, Response},
    v4::SocksStream,
    ReadTokio, WriteTokio,
};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::TcpListener, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example of a simple async SOCKS4 server using Tokio");

    let listener = TcpListener::bind("127.0.0.1:1080").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("New TCP stream accepted");

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 65535];

            let request = match SocksStream::read_async(&mut stream, &mut buffer).await {
                Ok(request) => request,
                Err(e) => {
                    eprintln!("Error reading SOCKS4 request: {:?}", e);
                    return;
                }
            };

            println!("Received SOCKS4 request: {:?}", request);

            let ip = request.get_ip();
            let port = request.get_port();
            let addr: SocketAddr = SocketAddr::new(ip, port);

            println!("Connecting to {}:{}", ip, port);

            let mut connection = match TcpStream::connect(addr).await {
                Ok(connection) => connection,
                Err(e) => {
                    eprintln!("Failed to connect to destination: {:?}", e);

                    return;
                }
            };

            let response = Response::new(Reply::Granted);
            if let Err(e) = SocksStream::write_async(&mut stream, response).await {
                eprintln!("Error sending response: {:?}", e);

                return;
            }

            let (mut stream_read, mut stream_write) = stream.split();
            let (mut connection_read, mut connection_write) = connection.split();

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

            println!("SOCKS4 connection closed");
        });
    }
}
