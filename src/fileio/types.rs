use std::os::fd::OwnedFd;
use nix::{
    fcntl::OFlag,
    libc::stat,
    sys::stat::Mode
};

pub struct FileIO {
    pub(crate) file: OwnedFd
}

pub struct FileIOBuilder {
    pub(crate) flags: OFlag,
    pub(crate) permissions: Mode
}

pub struct FileIOMetadata {
    pub(crate) stat: stat
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileIOType {
    Regular,
    Symlink,
    Directory,
    BlockDevice,
    CharDevice,
    Socket,
    MaskType,
    NamedPipe,
}