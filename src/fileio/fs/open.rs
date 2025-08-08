use std::os::fd::OwnedFd;
use std::path::PathBuf;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use std::io::Error as IoError;
use crate::errors::{Error, Result};
use crate::fileio::FileIO;

pub trait Open<T>: Sized
where
    T: AsRef<PathBuf>,
{
    type Error;
    
    fn open(path: T, flags: OFlag, perms: Mode) -> Result<Self>;
}

impl<T> Open<T> for FileIO
where
    T: AsRef<PathBuf>,
{
    type Error = Error;

    fn open(path: T, flags: OFlag, perms: Mode) -> Result<Self> {
        let file: OwnedFd = open(
            path.as_ref(),
            flags,
            perms
        ).map_err(|e| Error::Io(IoError::from_raw_os_error(e as i32)))?;

        return Ok(FileIO {
            file: file,
        });
    }
}