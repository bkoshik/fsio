use std::os::fd::AsRawFd;
use crate::error::*;
use crate::file::File;
use crate::prelude::Write;
use crate::syscall;
use crate::syscall::Syscall;

impl<B> Write<B> for File
where
    B: AsRef<str>,
{
    fn write(&mut self, buf: B) -> Result<usize> {
        let bytes = buf.as_ref().as_bytes();

        let written_len = syscall!(
            Syscall::Write, 
            self.as_raw_fd(),
            bytes.as_ptr() as *const libc::c_void,
            bytes.len()
        ) as usize;
        return Error::result(written_len);
    }
}