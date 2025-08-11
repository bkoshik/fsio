use crate::file::FileMetadata;

impl FileMetadata {
    pub fn uid(&self) -> u32 {
        return self.stat.st_uid
    }
    
    pub fn gid(&self) -> u32 {
        return self.stat.st_gid
    }
}