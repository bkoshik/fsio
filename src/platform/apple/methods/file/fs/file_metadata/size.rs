use crate::file::FileMetadata;

impl FileMetadata {
    pub fn size(&self) -> i64 {
        return self.metadata.st_size;
    }
}
