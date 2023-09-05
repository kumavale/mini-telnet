use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:2323").await.unwrap();
    let (sender, receiver) = broadcast::channel(1);
    let (stream, sink) = stream.into_split();

    let input_handle = stdin(sender);
    let tx_handle = tx(sink, receiver);
    let rx_handle = rx(stream);

    tokio::select! {
        _ = input_handle => (),
        _ = tx_handle => (),
        _ = rx_handle => (),
    }
}

fn stdin(sender: broadcast::Sender<Vec<u8>>) -> JoinHandle<()> {
    tokio::task::spawn_blocking(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        sender.send(input.into_bytes()).unwrap();
    })
}

async fn tx(mut sink: OwnedWriteHalf, mut proxy: broadcast::Receiver<Vec<u8>>) {
    loop {
        let input = proxy.recv().await.unwrap();
        sink.write_all(&input).await.unwrap();
    }
}

async fn rx(mut stream: OwnedReadHalf) {
    loop {
        let mut buf = vec![];
        match stream.read_buf(&mut buf).await {
            Ok(0) | Err(_) => return,
            Ok(_) => std::io::stdout().lock().write_all(&buf).unwrap(),
        }
    }
}
