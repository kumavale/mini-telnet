use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:2323")?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        let mut buf = [0; 128];
        let n = stream.read(&mut buf)?;
        std::io::stdout().write_all(&buf[..n])?;
    }
}
