use std::io::{
    Error as IoError,
    ErrorKind
};
use nix::unistd::lseek;
use crate::{
    errors::*,
    fileio::{
        FileIO,
        flags::whence_flags::CURRENT_POS
    }
};

impl FileIO {
    pub fn tell(&mut self) -> Result<usize> {
        return Ok(
            lseek(&self.file, 0, CURRENT_POS)
               .map_err(
                   |e| Error::Io(IoError::new(ErrorKind::InvalidData, e))
               )? as usize
       );
    }
}