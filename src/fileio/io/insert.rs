use std::io::Error;
use crate::{
    errors::*,
    fileio::{
        FileIO,
        flags::whence_flags::START_POS,
        prelude::Write
    }
};

pub trait Insert<T>: Sized {
    type Error;

    fn insert(&mut self, offset: isize, buf: T) -> Result<usize>;
}

impl<T> Insert<T> for FileIO
where
    T: AsRef<str>
{
    type Error = Error;

    fn insert(&mut self, offset: isize, buf: T) -> Result<usize> {
        let _ = self.seek(offset, START_POS)?;
        let data_from_offset = self.read()?;

        let _ = self.seek(offset, START_POS)?;
        let _ = self.write(format!("{}{}", buf.as_ref(), data_from_offset))?;

        return Ok(self.metadata()?.size as usize);
    }
}