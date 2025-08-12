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
        ).map_err(|e| Error::Errno(e))?;

        return Ok(File {
            file: file,
        });
    }
}