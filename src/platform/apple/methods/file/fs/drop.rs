use crate::file::File;
use crate::syscall;
use crate::syscall::Syscall;

impl Drop for File {
    fn drop(&mut self) {
        syscall!(Syscall::Close, self.file);
    }
}
