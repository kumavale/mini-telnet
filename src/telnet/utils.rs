use super::command;
use super::option;
use match_to_str::match_to_str;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

macro_rules! flat_vec {
    ( $( $ary:expr ),+ $(,)? ) => [
        [$( &$ary[..] ),*].concat()
    ]
}
pub(crate) use flat_vec;

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

pub fn get_window_size() -> [u8; 4] {
    use nix::libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
    use std::mem;

    let fd = STDOUT_FILENO;
    let mut ws: winsize = unsafe { mem::zeroed() };

    if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        return [0, 80, 0, 24];
    }

    let width: u16 = ws.ws_col;
    let height: u16 = ws.ws_row;

    flat_vec![width.to_be_bytes(), height.to_be_bytes()]
        .try_into()
        .unwrap()
}
