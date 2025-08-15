use std::os::fd::AsRawFd;
use crate::file::File;
use crate::error::*;
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn read(&self) -> Result<(String, usize)> {
        let mut content_bytes = vec![0u8; self.metadata()?.size() as usize];

        let read_len = syscall!(
            Syscall::Read,
            self.as_raw_fd(),
            content_bytes.as_mut_ptr() as *mut libc::c_void,
            content_bytes.len()
        ) as usize;
        Error::result(read_len)?;

        content_bytes.truncate(read_len);
        let content = String::from_utf8(content_bytes.to_vec())
            .map_err(|_| Error::IllegalByteSequence)?;

        return Ok((content, read_len));
    }
}