use std::os::fd::AsRawFd;
use libc::write;
use crate::error::*;
use crate::file::File;

pub trait Write<T>: Sized
where
    T: AsRef<str>
{
    fn write(&mut self, buf: T) -> Result<usize>;
}

impl<T> Write<T> for File
where
    T: AsRef<str>,
{
    fn write(&mut self, buf: T) -> Result<usize> {
        let bytes = buf.as_ref().as_bytes();

        let written_len = unsafe {
            write(self.as_raw_fd(), bytes.as_ptr().cast(), bytes.len()) as usize
        };
        return Error::result(written_len);
    }
}