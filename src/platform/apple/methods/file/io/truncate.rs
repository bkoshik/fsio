use std::os::fd::AsRawFd;
use crate::error::*;
use crate::file::File;
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn truncate(&self, len: usize) -> Result<usize> {
        let file_len = syscall!(Syscall::Ftruncate, self.as_raw_fd(), len) as usize;

        return Error::result(file_len);
    }
}