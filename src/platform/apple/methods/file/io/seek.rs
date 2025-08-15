use std::os::fd::AsRawFd;
use crate::error::*;
use crate::file::{File, SeekWhence};
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn seek(&self, whence: SeekWhence) -> Result<usize> {
        let offset = match whence {
            SeekWhence::StartPos(off) => syscall!(Syscall::Lseek, self.as_raw_fd(), off, 0),
            SeekWhence::CurrentPos(off) => syscall!(Syscall::Lseek, self.as_raw_fd(), off, 1),
            SeekWhence::EndPos(off) => syscall!(Syscall::Lseek, self.as_raw_fd(), off, 1),
        } as usize;
        
        return Error::result(offset)
    }
}