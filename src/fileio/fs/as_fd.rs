use std::os::fd::{AsFd, BorrowedFd};
use crate::fileio::FileIO;

impl AsFd for FileIO {
    fn as_fd(&self) -> BorrowedFd<'_> {
        return self.file.as_fd();
    }
}