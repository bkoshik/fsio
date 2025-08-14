use std::ffi::CString;
use std::os::fd::{FromRawFd, OwnedFd};
use libc::c_int;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::error::*;
use crate::file::File;

pub trait OpenFS<P>: Sized
where
    P: AsRef<str>,
{
    fn open(path: P, open_flags: OpenFlags, perms: PermissionFlags) -> Result<Self>;
}

impl<P> OpenFS<P> for File
where
    P: AsRef<str>,
{
    fn open(path: P, open_flags: OpenFlags, perms: PermissionFlags) -> Result<Self> {
        let c_path = CString::new(path.as_ref())
            .map_err(|_| Error::InvalidArgument)?;

        let raw_fd = unsafe {
            libc::open(c_path.as_ptr(), open_flags.bits() as c_int, perms.bits())
        };
        Error::result(raw_fd)?;

        return Ok(Self {
            file: unsafe { OwnedFd::from_raw_fd(raw_fd) }
        })
    }
}