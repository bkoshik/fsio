use crate::file::FileMetadata;

impl FileMetadata {
    pub fn uid(&self) -> u64 {
        return self.metadata.st_uid as u64;
    }

    pub fn gid(&self) -> u64 {
        return self.metadata.st_gid as u64;
    }
}
