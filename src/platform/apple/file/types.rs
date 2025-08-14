use std::os::fd::OwnedFd;
use libc::stat;

#[derive(Debug)]
pub struct File {
    pub(crate) file: OwnedFd
}

pub struct FileMetadata {
    pub(crate) metadata: stat
}