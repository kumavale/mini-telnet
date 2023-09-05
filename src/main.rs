use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("1984.ws:23").await?;
    let (sender, receiver) = broadcast::channel(1);
    let (mut stream, mut sink) = stream.into_split();

    negotiation(&mut stream, &mut sink).await?;

    let input_handle = stdin(sender);
    let tx_handle = tx(sink, receiver);
    let rx_handle = rx(stream);

    tokio::select! {
        r = input_handle => r?,
        r = tx_handle => r,
        r = rx_handle => r,
    }
}

fn stdin(sender: broadcast::Sender<Vec<u8>>) -> JoinHandle<anyhow::Result<()>> {
    tokio::task::spawn_blocking(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        sender.send(input.into_bytes())?;
    })
}

async fn tx(
    mut sink: OwnedWriteHalf,
    mut proxy: broadcast::Receiver<Vec<u8>>,
) -> anyhow::Result<()> {
    loop {
        let input = proxy.recv().await?;
        sink.write_all(&input).await?;
    }
}

async fn rx(mut stream: OwnedReadHalf) -> anyhow::Result<()> {
    loop {
        let mut buf = vec![];
        match stream.read_buf(&mut buf).await {
            Ok(0) | Err(_) => return Ok(()),
            Ok(_) => {
                std::io::stdout().lock().write_all(&buf)?;
                std::io::stdout().flush()?;
            }
        }
    }
}

async fn negotiation(stream: &mut OwnedReadHalf, sink: &mut OwnedWriteHalf) -> anyhow::Result<()> {
    // options
    const SUPPRESS_GO_AHEAD: u8 =  3;
    const WINDOW_SIZE:       u8 = 31;
    // commands
    const SE:   u8 = 240;
    const SB:   u8 = 250;
    const WILL: u8 = 251;
    const WONT: u8 = 252;
    const DO:   u8 = 253;
    const DONT: u8 = 254;
    const IAC:  u8 = 255;

    // My negotiation
    sink.write_all(&[IAC, WILL, WINDOW_SIZE]).await?;

    // Server negotiation
    loop {
        let mut buf = vec![0; 3];
        match stream.peek(&mut buf).await {
            Ok(0) => return Ok(()),
            Ok(_) => {
                if buf[0] == IAC {
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
                    stream.read_exact(&mut buf).await?;
                } else {
                    return Ok(()); // End of Negotiation
                }
            }
            Err(e) => anyhow::bail!(e),
        }
    }
}
