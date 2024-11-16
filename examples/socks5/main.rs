/*!
This is a simple SOCKS5 server example using threads.

# Usage

## HTTPie

You can use HTTPie with `--proxy` flag to do an HTTP request through the SOCKS server.

```bash
http --proxy=http:socks5://localhost:1080 http://example.com
```

## Chromium

You can also test the server through a Chromium based browser.

```bash
chromium --proxy-server="socks5://localhost:1080"
```
*/

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

use socks::{
    v5::{
        client::{Greeting, Request},
        server::Choice,
    },
    Version,
};

fn main() {
    println!("example of a simple SOCKS5 server");

    let listener = TcpListener::bind("127.0.0.1:1080").unwrap();
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(mut stream) => {
                println!("New TCP stream accepted");

                thread::spawn(move || {
                    println!("new tcp stream");

                    let mut buffer: Vec<u8> = vec![0 as u8; 65535];

                    let read = stream.read(&mut buffer).unwrap();
                    let greeting = Greeting::from(Vec::from(&buffer[..read]));
                    dbg!(greeting);

                    let choice = Choice {
                        version: Version::V5 as u8,
                        choose: 0,
                    };

                    let wrotten = stream.write(&[5, 0]).unwrap();
                    dbg!(choice);

                    let read = stream.read(&mut buffer);
                    if let Err(e) = read {
                        dbg!(e);

                        return;
                    }

                    let size = read.unwrap();

                    if size == 0 {
                        return;
                    }

                    let request = Request::from(buffer[..size].to_vec());

                    println!("Received SOCKS5 request: {:?}", request);

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

                    let response = Vec::from(&[
                        0x05,
                        0x00,
                        0x00,
                        1,
                        request.ip[0],
                        request.ip[1],
                        request.ip[2],
                        request.ip[3],
                        request.port[0],
                        request.port[1],
                    ]);

                    if let Err(e) = stream.write(&response) {
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

                    println!("SOCKS5 connection closed");
                });
            }
            Err(e) => {
                dbg!(e);

                return;
            }
        }
    }
}
