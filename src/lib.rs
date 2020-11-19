use std::convert::TryFrom;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll, Poll::*};

use tokio::io::{unix, AsyncRead, AsyncWrite, ReadBuf};

use self::fd::Fd;

mod fd;

pub struct AsyncFd(unix::AsyncFd<Fd>);

impl TryFrom<RawFd> for AsyncFd {
    type Error = Error;

    fn try_from(fd: RawFd) -> Result<Self> {
        let mut fd = Fd(fd);
        fd.set_nonblock()?;
        Ok(Self(unix::AsyncFd::new(fd)?))
    }
}

impl AsRawFd for AsyncFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.get_ref().0
    }
}

impl AsyncRead for AsyncFd {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let mut fd = Fd(self.as_raw_fd());
        let mut ready = match self.0.poll_read_ready(cx) {
            Ready(x) => x?,
            Pending => return Pending,
        };

        match unsafe {
            fd.read(&mut *(buf.unfilled_mut() as *mut _ as *mut [u8]))
                .map(|n| {
                    buf.assume_init(n);
                    buf.advance(n);
                })
        } {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                ready.clear_ready();
                Pending
            }
            x => Ready(x),
        }
    }
}

impl AsyncWrite for AsyncFd {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        let mut fd = Fd(self.as_raw_fd());
        let mut ready = match self.0.poll_write_ready(cx) {
            Ready(x) => x?,
            Pending => return Pending,
        };

        match fd.write(buf) {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                ready.clear_ready();
                Pending
            }
            x => Ready(x),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Ready(Ok(()))
    }
}
