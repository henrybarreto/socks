/*!
This is a simple SOCKS4 server example using threads.

# Usage

## HTTPie

You can use HTTPie with `--proxy` flag to do an HTTP request through the SOCKS server.

```bash
http --proxy=http:socks4://localhost:1080 http://example.com
```
*/

use ::socks::v4::{server::Response, Reply};
use socks::v4::socks;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    println!("example of a simple SOCKS4 server");

    let server = socks::Socks::new();

    server
        .listen("127.0.0.1:1080", |mut stream, _| async move {
            let reply = Reply::Granted;

            let response = Response::new(reply.clone());
            let response_buffer: Vec<u8> = response.into();

            let wrote = stream.write(&response_buffer).await;
            if let Err(e) = wrote {
                return Err(e);
            }

            let size = wrote.unwrap();
            if size == 0 {}

            if let Reply::Granted = reply {
                println!("Access allowed");
            } else {
                eprintln!("Access not allowed");
            }

            return Ok(stream);
        })
        .await
        .unwrap();
}
