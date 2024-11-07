/*!
This is a simple SOCKS4 server example using threads.

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

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

use socks::{
    request::Request,
    response::{Reply, Response},
    v4::SocksStream,
    Read as SocksRead, Write as SocksWrite,
};

fn main() {
    println!("example of a simple SOCKS4 server");

    let listener = TcpListener::bind("127.0.0.1:1080").unwrap();
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(mut stream) => {
                println!("New TCP stream accepted");

                thread::spawn(move || {
                    println!("new tcp stream");

                    let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                    let request: Request = match SocksStream::read(&mut stream, &mut buffer) {
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

                    let mut connection = match TcpStream::connect(addr) {
                        Ok(connection) => connection,
                        Err(e) => {
                            eprintln!("Failed to connect to destination: {:?}", e);

                            return;
                        }
                    };

                    let response = Response::new(Reply::Granted);
                    if let Err(e) = SocksStream::write(&mut stream, response) {
                        eprintln!("Error sending response: {:?}", e);

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

                    println!("SOCKS4 connection closed");
                });
            }
            Err(e) => {
                dbg!(e);

                return;
            }
        }
    }
}
