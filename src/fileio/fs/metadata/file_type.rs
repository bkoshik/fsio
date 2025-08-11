use nix::sys::stat::SFlag;
use crate::fileio::{FileIOMetadata, FileIOType};

impl FileIOMetadata {
    pub fn file_type(&self) -> Option<FileIOType> {
        return match SFlag::from_bits_truncate(self.stat.st_mode) {
            mode if mode == SFlag::S_IFREG => Some(FileIOType::Regular),
            mode if mode == SFlag::S_IFLNK => Some(FileIOType::Symlink),
            mode if mode == SFlag::S_IFDIR => Some(FileIOType::Directory),
            mode if mode == SFlag::S_IFBLK => Some(FileIOType::BlockDevice),
            mode if mode == SFlag::S_IFCHR => Some(FileIOType::CharDevice),
            mode if mode == SFlag::S_IFSOCK => Some(FileIOType::Socket),
            mode if mode == SFlag::S_IFMT => Some(FileIOType::MaskType),
            mode if mode == SFlag::S_IFIFO => Some(FileIOType::NamedPipe),
            _ => None
        }
    }
}