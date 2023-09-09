use super::command;
use super::option;
use tokio::io::AsyncReadExt;
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
            self.read_u8().await?;
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
        match *self {
            option::ECHO                => "ECHO",
            option::SUPPRESS_GO_AHEAD   => "SUPPRESS_GO_AHEAD",
            option::STATUS              => "STATUS",
            option::TERMINAL_TYPE       => "TERMINAL_TYPE",
            option::WINDOW_SIZE         => "WINDOW_SIZE",
            option::TERMINAL_SPEED      => "TERMINAL_SPEED",
            option::REMOTE_FLOW_CONTROL => "REMOTE_FLOW_CONTROL",
            option::LINE_MODE           => "LINE_MODE",
            option::X_DISPLAY_LOCATION  => "X_DISPLAY_LOCATION",
            option::ENVIRONMENT         => "ENVIRONMENT",
            option::AUTHENTICATION      => "AUTHENTICATION",
            option::ENCRYPT             => "ENCRYPT",
            option::NEW_ENVIRONMENT     => "NEW_ENVIRONMENT",
            _ => "...",
        }
    }

    fn command(&self) -> &'static str {
        match *self {
            command::SE   => "SE",
            command::SB   => "SB",
            command::WILL => "WILL",
            command::WONT => "WONT",
            command::DO   => "DO",
            command::DONT => "DONT",
            command::IAC  => "IAC",
            _ => "...",
        }
    }
}
