use crate::{
    errors::Result,
    prelude::OpenFS,
    fileio::{
        FileIO,
        builder::FileIOBuilder
    }
};

pub trait OpenBuilder<T>: Sized
where
    T: AsRef<str>
{
    fn open(&self, path: T) -> Result<FileIO>;
}

impl<T> OpenBuilder<T> for FileIOBuilder
where
    T: AsRef<str>
{
    fn open(&self, path: T) -> Result<FileIO> {
        return FileIO::open(path.as_ref(), self.flags, self.permissions);
    }
}