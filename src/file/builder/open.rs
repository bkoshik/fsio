use crate::{
    errors::Result,
    file::{
        prelude::OpenFS,
        File,
        FileBuilder
    }
};

pub trait OpenBuilder<T>: Sized
where
    T: AsRef<str>
{
    fn open(&self, path: T) -> Result<File>;
}

impl<T> OpenBuilder<T> for FileBuilder
where
    T: AsRef<str>
{
    fn open(&self, path: T) -> Result<File> {
        return File::open(path.as_ref(), self.flags, self.permissions);
    }
}