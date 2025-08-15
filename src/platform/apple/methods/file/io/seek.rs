use crate::error::*;
use crate::file::{File, SeekWhence};
use crate::syscall;
use crate::syscall::Syscall;
use std::os::fd::AsRawFd;

impl File {
    pub fn seek(&self, whence: SeekWhence) -> Result<usize> {
        let offset = {
            let ret = match whence {
                SeekWhence::StartPos(off) => syscall!(Syscall::Lseek, self.as_raw_fd(), off, 0),
                SeekWhence::CurrentPos(off) => syscall!(Syscall::Lseek, self.as_raw_fd(), off, 1),
                SeekWhence::EndPos(off) => syscall!(Syscall::Lseek, self.as_raw_fd(), off, 2),
            };
            Error::result(ret);

            ret as usize
        };

        return Ok(offset);
    }
}
