use std::os::fd::AsRawFd;
use crate::file::{File, FileMetadata};
use crate::error::{Error, Result};

impl File {
    pub fn metadata(&self) -> Result<FileMetadata> {
        let mut buf = std::mem::MaybeUninit::<libc::stat>::uninit();
        let metadata = unsafe {
            libc::fstat(self.as_raw_fd(), buf.as_mut_ptr())
        };
        Error::result(metadata)?;

        return Ok(FileMetadata {
            metadata: unsafe { buf.assume_init() }
        })
    }
}