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
    file::{
        File,
        FileMetadata
    }
};

impl File {
    pub fn metadata(&self) -> crate::errors::Result<FileMetadata> {
        let metadata = fstat(self.as_fd())
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidData, e)))?;

        Ok(FileMetadata {
            stat: metadata
        })
    }
}