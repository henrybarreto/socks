/*!
This is a simple SOCKS5 server example using threads.

# Usage

## HTTPie

You can use HTTPie with `--proxy` flag to do an HTTP request through the SOCKS server.

```bash
http --proxy=http:socks5://localhost:1080 http://example.com
```
*/

use std::io::Error;

use ::socks::v5::{
    client::{Greeting, Request},
    server::Choice,
    socks::Handler,
    Reply,
};
use socks::v5::socks;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

struct Example;

impl Example {
    fn new() -> Self {
        Example
    }
}

impl Handler for Example {
    fn auth(&self, _: Greeting) -> Result<Choice, Error> {
        Ok(Choice {
            version: 0x05,
            choose: 0,
        })
    }
    fn request(&self, _: Request) -> Result<Reply, Error> {
        Ok(Reply::RequestGranted)
    }
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("example of a simple SOCKS5 server listening on :1080");
    info!("all requests will be granted");

    let server = socks::Socks::new(Example::new());

    server.listen("0.0.0.0:1080").await.unwrap();
}
