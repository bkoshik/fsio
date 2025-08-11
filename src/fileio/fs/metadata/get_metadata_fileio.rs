use std::{
    os::fd::AsFd,
    io::{
        Error as IoError,
        ErrorKind
    }
};
use nix::sys::stat::fstat;
use crate::{
    errors::Error,
    fileio::{
        FileIO,
        fs::metadata::FileIOMetadata
    }
};

impl FileIO {
    pub fn metadata(&self) -> crate::errors::Result<FileIOMetadata> {
        let metadata = fstat(self.as_fd())
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidData, e)))?;

        Ok(FileIOMetadata {
            stat: metadata
        })
    }
}