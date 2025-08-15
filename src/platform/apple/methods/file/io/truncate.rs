use crate::error::*;
use crate::file::File;
use crate::syscall;
use crate::syscall::Syscall;
use std::os::fd::AsRawFd;

impl File {
    pub fn truncate(&self, len: u64) -> Result<u64> {
        let file_len = {
            let ret = syscall!(Syscall::Ftruncate, self.as_raw_fd(), len);
            Error::result(ret)?;

            ret as u64
        };

        return Ok(file_len);
    }
}
