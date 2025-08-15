use crate::error::*;
use crate::file::File;
use crate::syscall::*;

impl File {
    pub fn read(&self) -> Result<(String, u64)> {
        let mut content_bytes = vec![0u8; self.metadata()?.size() as usize];

        let read_len = unsafe {
            let mut args = [0i64; 6];
            args[0] = self.file as i64;
            args[1] = content_bytes.as_mut_ptr() as i64;
            args[2] = content_bytes.len() as i64;

            let ret = syscall(Syscall::Read, &args);
            Error::result(ret)?;

            ret as u64
        };

        content_bytes.truncate(read_len as usize);
        let content =
            String::from_utf8(content_bytes.to_vec()).map_err(|_| Error::IllegalByteSequence)?;

        return Ok((content, read_len));
    }
}
