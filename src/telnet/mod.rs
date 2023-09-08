use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

/// Options
#[rustfmt::skip]
mod option {
    pub const SUPPRESS_GO_AHEAD: u8 =  3;
    pub const WINDOW_SIZE:       u8 = 31;
}

/// Commands
#[rustfmt::skip]
mod command {
    pub const SE:   u8 = 240;
    pub const SB:   u8 = 250;
    pub const WILL: u8 = 251;
    pub const WONT: u8 = 252;
    pub const DO:   u8 = 253;
    pub const DONT: u8 = 254;
    pub const IAC:  u8 = 255;
}

pub async fn negotiation(
    stream: &mut OwnedReadHalf,
    sink: &mut OwnedWriteHalf,
) -> anyhow::Result<()> {
    // My init negotiation
    sink.write_all(&[command::IAC, command::WILL, option::WINDOW_SIZE])
        .await?;

    // Server negotiation
    loop {
        let mut buf = vec![0; 3];
        match stream.peek(&mut buf).await {
            Ok(0) => return Ok(()),
            Ok(_) => {
                if buf[0] == command::IAC {
                    if buf[1] == command::DO {
                        if buf[2] == option::WINDOW_SIZE {
                            buf = vec![
                                command::IAC,
                                command::SB,
                                option::WINDOW_SIZE,
                                0,
                                80,
                                0,
                                24,
                                command::IAC,
                                command::SE,
                            ];
                        } else {
                            buf[1] = command::WONT
                        }
                    }
                    if buf[1] == command::WILL {
                        if buf[2] == option::SUPPRESS_GO_AHEAD {
                            buf[1] = command::DO
                        } else {
                            buf[1] = command::DONT
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
