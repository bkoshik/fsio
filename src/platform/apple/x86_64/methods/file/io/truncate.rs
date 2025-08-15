use crate::error::*;
use crate::file::File;
use crate::syscall::*;

impl File {
    pub fn truncate(&self, len: u64) -> Result<u64> {
        let file_len = {
            let mut args = [0i64; 6];
            args[0] = self.file as i64;
            args[1] = len as i64;

            let ret = syscall(Syscall::Ftruncate, &args)?;

            ret as u64
        };

        return Ok(file_len);
    }
}
