use crate::error::*;
use crate::file::{File, SeekWhence};
use crate::syscall::*;

impl File {
    pub fn seek(&self, whence: SeekWhence) -> Result<u64> {
        let mut args = [0i64; 6];
        args[0] = self.file as i64;

        let offset = {
            let ret = match whence {
                SeekWhence::StartPos(off) => unsafe {
                    args[1] = off as i64;
                    args[2] = 0;

                    syscall(Syscall::Lseek, &args)
                },
                SeekWhence::CurrentPos(off) => unsafe {
                    args[1] = off;
                    args[2] = 1;

                    syscall(Syscall::Lseek, &args)
                },
                SeekWhence::EndPos(off) => unsafe {
                    args[1] = off;
                    args[2] = 2;

                    syscall(Syscall::Lseek, &args)
                },
            };
            Error::result(ret)?;

            ret as u64
        };

        return Ok(offset);
    }
}
