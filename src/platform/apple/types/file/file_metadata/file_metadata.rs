use libc::stat;

pub struct FileMetadata {
    pub(crate) metadata: stat
}