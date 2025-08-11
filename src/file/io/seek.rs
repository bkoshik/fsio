use std::io::{
    Error as IoError, 
    ErrorKind
};
use nix::{
    libc::off_t,
    unistd::lseek,
    unistd::Whence
};
use crate::{
    errors::*,
    file::File
};

impl File {
    pub fn seek(&mut self, offset: isize, whence: Whence) -> Result<isize> {
        return Ok(
            lseek(&self.file, offset as off_t, whence)
                .map_err(
                    |e| Error::Io(IoError::new(ErrorKind::InvalidData, e))
                )? as isize
        );
    }
}