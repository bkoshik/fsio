use std::os::fd::AsFd;
use std::io::{Error as IoError, ErrorKind};
use nix::sys::stat::{fstat, SFlag};
use crate::errors::{Error, Result};
use crate::fileio::FileIO;

pub struct FileMetadata {
    pub file_type: SFlag,
    pub size: u64,
    pub accessed_time: i64,
    pub modified_time: i64,
    pub permissions: u32,
    pub uid: u32,
    pub gid: u32,
}

impl FileIO {
    pub fn metadata(&self) -> Result<FileMetadata> {
        let metadata = fstat(self.as_fd())
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidData, e)))?;

        let filetype = SFlag::from_bits_truncate(metadata.st_mode);

        Ok(FileMetadata {
            file_type: filetype,
            size: metadata.st_size as u64,
            accessed_time: metadata.st_atime,
            modified_time: metadata.st_mtime,
            permissions: metadata.st_mode as u32,
            uid: metadata.st_uid,
            gid: metadata.st_gid,
        })
    }
}
