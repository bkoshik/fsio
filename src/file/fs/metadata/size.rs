use crate::file::FileMetadata;

impl FileMetadata {
    pub fn size(&self) -> usize {
        return self.stat.st_size as usize;
    }
}