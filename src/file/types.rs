use std::os::fd::OwnedFd;
use nix::{
    fcntl::OFlag,
    libc::stat,
    sys::stat::Mode
};

#[derive(Debug)]
pub struct File {
    pub(crate) file: OwnedFd
}

pub struct FileBuilder {
    pub(crate) flags: OFlag,
    pub(crate) permissions: Mode
}

pub struct FileMetadata {
    pub(crate) stat: stat
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Symlink,
    Directory,
    BlockDevice,
    CharDevice,
    Socket,
    MaskType,
    NamedPipe,
}