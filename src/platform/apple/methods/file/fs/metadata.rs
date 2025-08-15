use crate::error::{Error, Result};
use crate::file::{File, FileMetadata};
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn metadata(&self) -> Result<FileMetadata> {
        let mut buf = std::mem::MaybeUninit::<libc::stat>::uninit();
        let _ = {
            let ret = syscall!(Syscall::Fstat64, self.file, buf.as_mut_ptr());
            Error::result(ret)?;

            ret as u64
        };

        return Ok(FileMetadata {
            metadata: unsafe { buf.assume_init() },
        });
    }
}
