use std::io::{
    Error as IoError,
    ErrorKind
};
use regex::Regex;
use crate::{
    errors::*,
    prelude::Write,
    flags::whence_flags::START_POS,
    fileio::{
        FileIO,
    }
};

pub trait Replace<T> {
    type Error;

    fn replace(&mut self, from: T, to: T) -> Result<usize>;
}

impl<T> Replace<T> for FileIO
where
    T: AsRef<str>,
{
    type Error = Error;

    fn replace(&mut self, from: T, to: T) -> Result<usize> {
        let re = Regex::new(from.as_ref())
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidInput, e)))?;
        let _ = self.seek(0, START_POS);
        let data = self.read()?;

        let new_data = re.replace_all(&data, to.as_ref());
        let data_len = new_data.len();

        let _ = self.seek(0, START_POS);
        let _ = self.write(new_data)?;

        let _ = self.truncate(data_len)?;

        return Ok(data_len);
    }
}