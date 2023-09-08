use tokio::net::tcp::OwnedReadHalf;

#[async_trait::async_trait]
pub trait ReadStreamExt {
    async fn read_until(&mut self, end: u8) -> Result<Vec<u8>, std::io::Error>;
    async fn peek_u8(&mut self) -> Result<u8, std::io::Error>;
}

#[async_trait::async_trait]
impl ReadStreamExt for OwnedReadHalf {
    async fn read_until(&mut self, end: u8) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = vec![];
        loop {
            let peek = self.peek_u8().await?;
            buf.push(peek);
            if peek == end {
                return Ok(buf);
            }
        }
    }

    async fn peek_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut peek = [0];
        match self.peek(&mut peek).await? {
            0 => Err(std::io::Error::new(std::io::ErrorKind::Other, "EOF")),
            1 => Ok(peek[0]),
            _ => unreachable!(),
        }
    }
}
