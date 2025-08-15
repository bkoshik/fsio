use crate::file::{FileMetadata, FileType};
use crate::flags::FileTypeFlags;

impl FileMetadata {
    pub fn file_type(&self) -> Option<FileType> {
        return FileType::from_raw(self.metadata.st_mode as u64);
    }

    pub fn file_type_flags(&self) -> FileTypeFlags {
        return FileTypeFlags::from_bits_truncate(
            self.metadata.st_mode as u64 & FileTypeFlags::MaskType.bits(),
        );
    }
}
