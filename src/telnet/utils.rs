use super::command;
use super::option;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use match_to_str::match_to_str;

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
            self.read_u8().await?;
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

pub trait DisplayExt {
    fn option(&self) -> &'static str;
    fn command(&self) -> &'static str;
}

#[rustfmt::skip]
impl DisplayExt for u8 {
    fn option(&self) -> &'static str {
        use option::*;
        match_to_str!(*self => {
            ECHO,
            SUPPRESS_GO_AHEAD,
            STATUS,
            TERMINAL_TYPE,
            WINDOW_SIZE,
            TERMINAL_SPEED,
            REMOTE_FLOW_CONTROL,
            LINE_MODE,
            X_DISPLAY_LOCATION,
            ENVIRONMENT,
            AUTHENTICATION,
            ENCRYPT,
            NEW_ENVIRONMENT,
            _,
        })
    }

    fn command(&self) -> &'static str {
        use command::*;
        match_to_str!(*self => {
            SE,
            SB,
            WILL,
            WONT,
            DO,
            DONT,
            IAC,
            _,
        })
    }
}
