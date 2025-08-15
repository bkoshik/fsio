use crate::file::File;
use std::os::fd::{FromRawFd, IntoRawFd, OwnedFd, RawFd};

impl From<RawFd> for File {
    fn from(fd: RawFd) -> Self {
        return Self { file: fd as u64 };
    }
}

impl From<File> for RawFd {
    fn from(file: File) -> Self {
        return file.file as i32;
    }
}

impl From<OwnedFd> for File {
    fn from(fd: OwnedFd) -> Self {
        return Self {
            file: fd.into_raw_fd() as u64,
        };
    }
}

impl From<File> for OwnedFd {
    fn from(file: File) -> Self {
        return unsafe { Self::from_raw_fd(file.into()) };
    }
}
