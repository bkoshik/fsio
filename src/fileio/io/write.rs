use std::{
    io::{
        Error as IoError,
        ErrorKind
    },
    os::fd::AsFd
};
use nix::unistd::write;
use crate::{
    errors::*,
    fileio::FileIO
};

pub trait Write<T>: Sized
where
    T: AsRef<str>
{
    fn write(&mut self, buf: T) -> Result<usize>;
}

impl<T> Write<T> for FileIO
where
    T: AsRef<str>,
{
    fn write(&mut self, buf: T) -> Result<usize> {
        let bytes = buf.as_ref().as_bytes();
        let mut written = 0;

        while written < bytes.len() {
            match write(&self.file.as_fd(), &bytes[written..]) {
                Ok(n) if n == 0 =>
                    return Err(Error::Io(IoError::new(ErrorKind::WriteZero, "write returned 0"))),
                Ok(n) => written += n,
                Err(e) => return Err(Error::Io(IoError::from(e))),
            }
        }

        return Ok(written);
    }
}