use std::os::fd::AsRawFd;
use nix::libc::{
    fflush,
    FILE
};
use crate::file::File;

impl File {
    pub fn flush(&self) -> usize {
        return unsafe { fflush(self.file.as_raw_fd() as *mut FILE) } as usize;
    }
}