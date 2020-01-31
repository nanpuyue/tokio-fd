use std::convert::TryFrom;
use std::io::{Error, Result};
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, PollEvented};

use self::mio::Fd;

mod mio;

pub struct AsyncFd(PollEvented<Fd>);

impl TryFrom<RawFd> for AsyncFd {
    type Error = Error;

    fn try_from(fd: RawFd) -> Result<Self> {
        let mut fd = Fd(fd);
        fd.set_nonblock()?;
        Ok(Self(PollEvented::new(fd)?))
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
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut Pin::into_inner(self).0).poll_read(cx, buf)
    }
}

impl AsyncWrite for AsyncFd {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        Pin::new(&mut Pin::into_inner(self).0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut Pin::into_inner(self).0).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut Pin::into_inner(self).0).poll_shutdown(cx)
    }
}
