use crate::flags::FileTypeFlags;

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
    pub fn from_raw(raw_mode: usize) -> Option<Self> {
        let bits = raw_mode & FileTypeFlags::MaskType.bits();

        return match FileTypeFlags::from_bits_truncate(bits) {
            FileTypeFlags::NamedPipe => Some(Self::NamedPipe),
            FileTypeFlags::CharacterDevice => Some(Self::CharacterDevice),
            FileTypeFlags::Directory => Some(Self::Directory),
            FileTypeFlags::BlockDevice => Some(Self::BlockDevice),
            FileTypeFlags::Regular => Some(Self::Regular),
            FileTypeFlags::Symlink => Some(Self::Symlink),
            FileTypeFlags::Socket => Some(Self::Socket),
            _ => None
        }
    }
}