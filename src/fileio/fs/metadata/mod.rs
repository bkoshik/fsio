mod file_type;
mod time;
mod size;

use std::{
    io::{
        Error as IoError,
        ErrorKind
    },
    os::fd::AsFd
};
use nix::{
    libc::stat,
    sys::stat::fstat,
};
use crate::{
    fileio::FileIO,
    errors::{
        Error,
        Result
    }
};

pub struct FileIOMetadata {
    stat: stat
}

impl FileIO {
    pub fn metadata(&self) -> Result<FileIOMetadata> {
        let metadata = fstat(self.as_fd())
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidData, e)))?;

        Ok(FileIOMetadata {
            stat: metadata
        })
    }
}
