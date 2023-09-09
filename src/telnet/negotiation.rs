use log::debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use super::command::*;
use super::option::*;
use super::utils::{DisplayExt, ReadStreamExt};

pub async fn negotiation(
    stream: &mut OwnedReadHalf,
    sink: &mut OwnedWriteHalf,
) -> anyhow::Result<()> {
    // My init negotiation
    sink.write_all(&[IAC, WILL, WINDOW_SIZE]).await?;

    // Server negotiation
    loop {
        let mut buf = vec![0; 3];
        match stream.peek(&mut buf).await? {
            0 => return Ok(()),
            _ => match buf[0] {
                IAC => match buf[1] {
                    WILL | DO | WONT | DONT => {
                        debug_assert_eq!(buf.len(), 3);
                        debug!(
                            "<--- {} {} {}",
                            buf[0].command(),
                            buf[1].command(),
                            buf[2].option(),
                        );
                        if buf[1] == DO {
                            if buf[2] == WINDOW_SIZE {
                                buf = vec![IAC, SB, WINDOW_SIZE, 0, 80, 0, 24, IAC, SE];
                            } else {
                                buf[1] = WONT;
                            }
                        }
                        if buf[1] == WILL {
                            if buf[2] == SUPPRESS_GO_AHEAD {
                                buf[1] = DO;
                            } else {
                                buf[1] = DONT;
                            }
                        }
                        stream.read_exact(&mut [0; 3]).await?;
                        sink.write_all(&buf).await?;
                        debug!(
                            "---> {} {} {} {:?}",
                            buf[0].command(),
                            buf[1].command(),
                            buf[2].option(),
                            &buf[3..],
                        );
                    }
                    SB => {
                        let buf = stream.read_until(SE).await?;
                        debug!(
                            "<--- {} {} {} {:?}",
                            buf[0].command(),
                            buf[1].command(),
                            buf[2].option(),
                            &buf[3..],
                        );
                    }
                    _ => unimplemented!(),
                },
                _ => return Ok(()), // End of Negotiation
            },
        }
    }
}
