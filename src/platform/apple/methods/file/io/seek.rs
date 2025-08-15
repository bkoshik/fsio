use crate::error::*;
use crate::file::{File, SeekWhence};
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn seek(&self, whence: SeekWhence) -> Result<u64> {
        let offset = {
            let ret = match whence {
                SeekWhence::StartPos(off) => syscall!(Syscall::Lseek, self.file, off, 0),
                SeekWhence::CurrentPos(off) => syscall!(Syscall::Lseek, self.file, off, 1),
                SeekWhence::EndPos(off) => syscall!(Syscall::Lseek, self.file, off, 2),
            };
            Error::result(ret)?;

            ret as u64
        };

        return Ok(offset);
    }
}
