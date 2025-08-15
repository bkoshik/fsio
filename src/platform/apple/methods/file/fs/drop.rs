use crate::file::File;
use crate::syscall::*;

impl Drop for File {
    fn drop(&mut self) {
        let mut args = [0i64; 6];
        args[0] = self.file as i64;

        let _ = syscall(Syscall::Close, &args).unwrap();
    }
}
