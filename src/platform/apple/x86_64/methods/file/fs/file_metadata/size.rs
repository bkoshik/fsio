use crate::file::FileMetadata;

impl FileMetadata {
    pub fn size(&self) -> u64 {
        return self.metadata.st_size as u64;
    }
}
