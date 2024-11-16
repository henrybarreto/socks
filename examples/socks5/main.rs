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
    v5::{server::Choice, Reply},
    Version,
};
use socks::v5::socks;

fn main() {
    println!("example of a simple SOCKS5 server");

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
            |_| {
                return Reply::RequestGranted;
            },
        )
        .unwrap();
}
