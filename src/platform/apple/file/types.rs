use crate::flags::FileTypeFlags;
use libc::stat;
use std::os::fd::OwnedFd;

#[derive(Debug)]
pub struct File {
    pub(crate) file: OwnedFd
}

pub struct FileMetadata {
    pub(crate) metadata: stat
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    NamedPipe,
    CharacterDevice,
    Directory,
    BlockDevice,
    Regular,
    Symlink,
    Socket,
}

impl FileType {
    pub fn from_raw(raw_mode: usize) -> Option<FileType> {
        let bits = raw_mode & FileTypeFlags::MaskType.bits();

        return match FileTypeFlags::from_bits_truncate(bits) {
            FileTypeFlags::NamedPipe => Some(FileType::NamedPipe),
            FileTypeFlags::CharacterDevice => Some(FileType::CharacterDevice),
            FileTypeFlags::Directory => Some(FileType::Directory),
            FileTypeFlags::BlockDevice => Some(FileType::BlockDevice),
            FileTypeFlags::Regular => Some(FileType::Regular),
            FileTypeFlags::Symlink => Some(FileType::Symlink),
            FileTypeFlags::Socket => Some(FileType::Socket),
            _ => None
        }
    }
}