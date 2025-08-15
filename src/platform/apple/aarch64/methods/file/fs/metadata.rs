use crate::error::Result;
use crate::file::{File, FileMetadata};
use crate::syscall::*;

impl File {
    pub fn metadata(&self) -> Result<FileMetadata> {
        let mut buf = std::mem::MaybeUninit::<libc::stat>::uninit();
        let _ = {
            let mut args = [0i64; 6];
            args[0] = self.file as i64;
            args[1] = buf.as_mut_ptr() as i64;

            let ret = syscall(Syscall::Fstat64, &args)?;

            ret as u64
        };

        return Ok(FileMetadata {
            metadata: unsafe { buf.assume_init() },
        });
    }
}
