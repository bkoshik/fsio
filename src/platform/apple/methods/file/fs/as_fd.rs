use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};
use crate::file::File;

impl AsFd for File {
    fn as_fd(&self) -> BorrowedFd<'_> {
        return self.file.as_fd()
    }
}

impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        return self.file.as_raw_fd()
    }
}