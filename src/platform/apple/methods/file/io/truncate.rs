use crate::error::*;
use crate::file::File;
use crate::syscall;
use crate::syscall::Syscall;
use std::os::fd::AsRawFd;

impl File {
    pub fn truncate(&self, len: usize) -> Result<usize> {
        let file_len = {
            let ret = syscall!(Syscall::Ftruncate, self.as_raw_fd(), len) as isize;
            Error::result(ret)?;

            ret as usize
        };

        return Ok(file_len);
    }
}
