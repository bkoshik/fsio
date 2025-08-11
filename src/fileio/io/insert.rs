use crate::{
    errors::*,
    prelude::Write,
    flags::whence_flags::START_POS,
    fileio::FileIO,
};

pub trait Insert<T>: Sized
where
    T: AsRef<str>
{
    fn insert(&mut self, offset: isize, buf: T) -> Result<usize>;
}

impl<T> Insert<T> for FileIO
where
    T: AsRef<str>
{
    fn insert(&mut self, offset: isize, buf: T) -> Result<usize> {
        let _ = self.seek(offset, START_POS)?;
        let data_from_offset = self.read()?;

        let _ = self.seek(offset, START_POS)?;
        let _ = self.write(format!("{}{}", buf.as_ref(), data_from_offset))?;

        return Ok(self.metadata()?.size());
    }
}