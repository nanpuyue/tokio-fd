use std::io::{Error, Read, Result, Write};
use std::os::unix::io::{AsRawFd, RawFd};

pub(crate) struct Fd(pub RawFd);

impl Fd {
    pub(crate) fn set_nonblock(&mut self) -> Result<()> {
        let flags = unsafe { libc::fcntl(self.0, libc::F_GETFL) };
        if flags < 0 {
            return Err(Error::last_os_error());
        }

        match unsafe { libc::fcntl(self.0, libc::F_SETFL, flags | libc::O_NONBLOCK) } {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

impl Read for Fd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let ret = unsafe { libc::read(self.0, buf.as_mut_ptr() as _, buf.len()) };
        if ret < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(ret as _)
        }
    }
}

impl Write for Fd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let ret = unsafe { libc::write(self.0, buf.as_ptr() as _, buf.len()) };
        if ret < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(ret as _)
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}
