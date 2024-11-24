/*!
This is a simple SOCKS5 server example using threads.

# Usage

## HTTPie

You can use HTTPie with `--proxy` flag to do an HTTP request through the SOCKS server.

```bash
http --proxy=http:socks5://localhost:1080 http://example.com
```
*/

use ::socks::{
    v5::{
        server::{Choice, Response},
        Reply,
    },
    Version,
};
use log::{error, info, warn};
use socks::v5::socks;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env("LOG")
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("example of a simple SOCKS5 server");

    let server = socks::Socks::new();

    server
        .listen(
            "127.0.0.1:1080",
            |_| {
                return Choice {
                    version: Version::V5 as u8,
                    choose: 0,
                };
            },
            |mut stream, request| async move {
                let reply = Reply::RequestGranted;

                let response = Response::new(reply.clone(), request.addr.to_vec(), request.port);
                let response_buffer: Vec<u8> = response.into();

                let wrote = stream.write(&response_buffer).await;
                if let Err(e) = wrote {
                    error!("error on response written to stream: {:?}", e);

                    return Err(e);
                }

                let size = wrote.unwrap();
                if size == 0 {
                    error!("nothing was written to response");
                }

                if let Reply::RequestGranted = reply {
                    info!("access allowed");
                } else {
                    warn!("access not allowed due {:?}", reply);
                }

                return Ok(stream);
            },
        )
        .await
        .unwrap();
}
