use std::{
    os::fd::OwnedFd,
    path::Path
};
use nix::{
    fcntl::{
        open,
        OFlag
    }, 
    sys::stat::Mode
};
use std::io::Error as IoError;
use crate::{
    errors::{
        Error,
        Result
    },
    file::File
};

pub trait OpenFS<T>: Sized
where
    T: AsRef<Path>,
{
    fn open(path: T, flags: OFlag, perms: Mode) -> Result<Self>;
}

impl<T> OpenFS<T> for File
where
    T: AsRef<Path>,
{
    fn open(path: T, flags: OFlag, perms: Mode) -> Result<Self> {
        let file: OwnedFd = open(
            path.as_ref(),
            flags,
            perms
        ).map_err(|e| Error::Io(IoError::from_raw_os_error(e as i32)))?;

        return Ok(File {
            file: file,
        });
    }
}