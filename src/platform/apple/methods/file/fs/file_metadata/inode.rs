use crate::file::FileMetadata;

impl FileMetadata {
    pub fn inode(&self) -> u64 {
        return self.metadata.st_ino
    }
}