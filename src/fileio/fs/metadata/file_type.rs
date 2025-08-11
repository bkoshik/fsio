use nix::sys::stat::SFlag;
use crate::fileio::fs::metadata::FileIOMetadata;

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

impl FileIOMetadata {
    pub fn file_type(&self) -> Option<FileIOType> {
        return match SFlag::from_bits(self.stat.st_mode)? {
            mode if mode.contains(SFlag::S_IFREG) => Some(FileIOType::Regular),
            mode if mode.contains(SFlag::S_IFLNK) => Some(FileIOType::Symlink),
            mode if mode.contains(SFlag::S_IFDIR) => Some(FileIOType::Directory),
            mode if mode.contains(SFlag::S_IFBLK) => Some(FileIOType::BlockDevice),
            mode if mode.contains(SFlag::S_IFCHR) => Some(FileIOType::CharDevice),
            mode if mode.contains(SFlag::S_IFSOCK) => Some(FileIOType::Socket),
            mode if mode.contains(SFlag::S_IFMT) => Some(FileIOType::MaskType),
            mode if mode.contains(SFlag::S_IFIFO) => Some(FileIOType::NamedPipe),
            _ => None
        }
    }
}