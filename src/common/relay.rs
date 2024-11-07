//! Data relay utilities for SOCKS protocol implementations.

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
};
use tracing::{error, trace, warn};

/// Statistics for data relay operations.
#[derive(Debug, Default)]
pub struct RelayStats {
    pub bytes_to_client: u64,
    pub bytes_to_target: u64,
    pub packets_to_client: u64,
    pub packets_to_target: u64,
}

impl RelayStats {
    /// Creates a new instance with zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Performs bidirectional data relay between two streams.
///
/// This function reads data from both streams and forwards it to the other stream.
/// It continues until either stream is closed or an error occurs.
pub async fn relay_data(mut stream_a: TcpStream, mut stream_b: TcpStream) -> RelayStats {
    let (mut a_read, mut a_write) = stream_a.split();
    let (mut b_read, mut b_write) = stream_b.split();

    let (mut buffer_a, mut buffer_b) = (vec![0u8; 65535], vec![0u8; 65535]);
    let mut stats = RelayStats::new();

    trace!("starting data relay between streams");

    loop {
        select! {
            Ok(size) = a_read.read(&mut buffer_a) => {
                if size == 0 {
                    trace!("stream A closed connection");

                    break;
                }

                stats.bytes_to_client += size as u64;
                stats.packets_to_client += 1;

                trace!(bytes = size, "relaying data from stream A to stream B");
                if let Err(e) = b_write.write_all(&buffer_a[..size]).await {
                    error!(error = ?e, "error writing to stream B");
                    break;
                }
            },
            Ok(size) = b_read.read(&mut buffer_b) => {
                if size == 0 {
                    trace!("stream B closed connection");
                    break;
                }

                stats.bytes_to_target += size as u64;
                stats.packets_to_target += 1;

                trace!(bytes = size, "relaying data from stream B to stream A");
                if let Err(e) = a_write.write_all(&buffer_b[..size]).await {
                    error!(error = ?e, "error writing to stream A");
                    break;
                }
            },
            else => {
                warn!("unexpected event in relay loop");
                break;
            },
        };
    }

    return stats;
}
