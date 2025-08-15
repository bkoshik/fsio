use std::os::fd::AsRawFd;
use libc::read;
use crate::file::File;
use crate::error::*;

impl File {
    pub fn read(&self) -> Result<String> {
        let mut content_bytes = vec![0u8; self.metadata()?.size() as usize];

        let data_len = unsafe {
            read(self.as_raw_fd(), content_bytes.as_mut_ptr().cast(), content_bytes.len()) as usize
        };
        Error::result(data_len)?;

        content_bytes.truncate(data_len);

        let content = String::from_utf8(content_bytes.to_vec())
            .map_err(|_| Error::IllegalByteSequence)?;

        return Ok(content);
    }
}