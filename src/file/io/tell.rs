use std::io::{
    Error as IoError,
    ErrorKind
};
use nix::unistd::lseek;
use crate::{
    errors::*,
    flags::whence_flags::CURRENT_POS,
    file::{
        File,
    }
};

impl File {
    pub fn tell(&self) -> Result<usize> {
        return Ok(
            lseek(&self.file, 0, CURRENT_POS)
               .map_err(
                   |e| Error::Io(IoError::new(ErrorKind::InvalidData, e))
               )? as usize
       );
    }
}