use std::os::fd::{AsFd, BorrowedFd};
use crate::file::File;

impl AsFd for File {
    fn as_fd(&self) -> BorrowedFd<'_> {
        return self.file.as_fd();
    }
}