use std::os::fd::AsRawFd;
use crate::file::{File, FileMetadata};
use crate::error::{Error, Result};
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn metadata(&self) -> Result<FileMetadata> {
        let mut buf = std::mem::MaybeUninit::<libc::stat>::uninit();
        let metadata = syscall!(
            Syscall::Fstat,
            self.as_raw_fd(), 
            buf.as_mut_ptr()
        );
        Error::result(metadata)?;

        return Ok(FileMetadata {
            metadata: unsafe { buf.assume_init() }
        })
    }
}