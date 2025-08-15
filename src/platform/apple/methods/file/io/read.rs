use crate::error::*;
use crate::file::File;
use crate::syscall;
use crate::syscall::Syscall;

impl File {
    pub fn read(&self) -> Result<(String, u64)> {
        let mut content_bytes = vec![0u8; self.metadata()?.size() as usize];

        let read_len = {
            let ret = syscall!(
                Syscall::Read,
                self.file,
                content_bytes.as_mut_ptr() as *mut libc::c_void,
                content_bytes.len()
            );
            Error::result(ret)?;

            ret as u64
        };

        content_bytes.truncate(read_len as usize);
        let content =
            String::from_utf8(content_bytes.to_vec()).map_err(|_| Error::IllegalByteSequence)?;

        return Ok((content, read_len));
    }
}
