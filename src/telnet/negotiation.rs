use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use super::command::*;
use super::option::*;

pub async fn negotiation(
    stream: &mut OwnedReadHalf,
    sink: &mut OwnedWriteHalf,
) -> anyhow::Result<()> {
    // My init negotiation
    sink.write_all(&[IAC, WILL, WINDOW_SIZE]).await?;

    // Server negotiation
    loop {
        let mut buf = vec![0; 3];
        match stream.peek(&mut buf).await {
            Ok(0) => return Ok(()),
            Ok(_) => {
                if buf[0] == IAC {
                    debug_assert_eq!(buf.len(), 3);
                    if buf[1] == DO {
                        if buf[2] == WINDOW_SIZE {
                            buf = vec![IAC, SB, WINDOW_SIZE, 0, 80, 0, 24, IAC, SE];
                        } else {
                            buf[1] = WONT
                        }
                    }
                    if buf[1] == WILL {
                        if buf[2] == SUPPRESS_GO_AHEAD {
                            buf[1] = DO
                        } else {
                            buf[1] = DONT
                        }
                    }
                    sink.write_all(&buf).await?;
                    stream.read_exact(&mut [0; 3]).await?;
                } else {
                    return Ok(()); // End of Negotiation
                }
            }
            Err(e) => anyhow::bail!(e),
        }
    }
}
