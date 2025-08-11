use crate::fileio::fs::metadata::FileIOMetadata;

impl FileIOMetadata {
    pub fn size(&self) -> usize {
        return self.stat.st_size as usize;
    }
}