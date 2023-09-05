mod telnet;

use clap::Parser;
use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// hostname:port
    hostname: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let stream = TcpStream::connect(&cli.hostname).await?;
    let (sender, receiver) = broadcast::channel(1);
    let (mut stream, mut sink) = stream.into_split();

    telnet::negotiation(&mut stream, &mut sink).await?;

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
