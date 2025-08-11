use crate::fileio::FileIOMetadata;

impl FileIOMetadata {
    pub fn uid(&self) -> u32 {
        return self.stat.st_uid
    }
    
    pub fn gid(&self) -> u32 {
        return self.stat.st_gid
    }
}