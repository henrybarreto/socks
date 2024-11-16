/*!
This is a simple SOCKS4 server example using threads.

# Usage

## HTTPie

You can use HTTPie with `--proxy` flag to do an HTTP request through the SOCKS server.

```bash
http --proxy=http:socks4://localhost:1080 http://example.com
```
*/

use ::socks::v4::Reply;
use socks::v4::socks;

fn main() {
    println!("example of a simple SOCKS4 server");

    let server = socks::Socks::new();

    server
        .listen("127.0.0.1:1080", |_| {
            return Reply::Granted;
        })
        .unwrap();
}
