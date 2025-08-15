use crate::error::*;
use crate::file::File;
use crate::prelude::Write;
use crate::syscall::*;

impl<B> Write<B> for File
where
    B: AsRef<str>,
{
    fn write(&mut self, buf: B) -> Result<u64> {
        let bytes = buf.as_ref().as_bytes();

        let written_len = {
            let mut args = [0i64; 6];
            args[0] = self.file as i64;
            args[1] = bytes.as_ptr() as i64;
            args[2] = bytes.len() as i64;

            let ret = syscall(Syscall::Write, &args)?;

            ret as u64
        };

        return Ok(written_len);
    }
}
