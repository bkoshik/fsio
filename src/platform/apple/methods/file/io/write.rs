use crate::error::*;
use crate::file::File;
use crate::prelude::Write;
use crate::syscall;
use crate::syscall::Syscall;

impl<B> Write<B> for File
where
    B: AsRef<str>,
{
    fn write(&mut self, buf: B) -> Result<u64> {
        let bytes = buf.as_ref().as_bytes();

        let written_len = {
            let ret = syscall!(
                Syscall::Write,
                self.file,
                bytes.as_ptr() as *const libc::c_void,
                bytes.len()
            );
            Error::result(ret)?;

            ret as u64
        };

        return Ok(written_len);
    }
}
