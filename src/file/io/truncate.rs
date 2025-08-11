use std::io::{
    Error as IoError,
    ErrorKind
};
use nix::{
    libc::off_t,
    unistd::ftruncate
};
use crate::{
    errors::*,
    file::File
};

impl File {
    pub fn truncate(&mut self, len: usize) -> Result<usize> {
        ftruncate(&mut self.file, len as off_t)
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidInput, e)))?;
        return Ok(self.metadata()?.size());
    }
}