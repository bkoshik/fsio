use nix::sys::stat::SFlag;
use crate::file::{FileMetadata, FileType};

impl FileMetadata {
    pub fn file_type(&self) -> Option<FileType> {
        return match SFlag::from_bits_truncate(self.stat.st_mode) {
            mode if mode == SFlag::S_IFREG => Some(FileType::Regular),
            mode if mode == SFlag::S_IFLNK => Some(FileType::Symlink),
            mode if mode == SFlag::S_IFDIR => Some(FileType::Directory),
            mode if mode == SFlag::S_IFBLK => Some(FileType::BlockDevice),
            mode if mode == SFlag::S_IFCHR => Some(FileType::CharDevice),
            mode if mode == SFlag::S_IFSOCK => Some(FileType::Socket),
            mode if mode == SFlag::S_IFMT => Some(FileType::MaskType),
            mode if mode == SFlag::S_IFIFO => Some(FileType::NamedPipe),
            _ => None
        }
    }
}