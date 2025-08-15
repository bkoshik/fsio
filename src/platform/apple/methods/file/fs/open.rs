use std::ffi::CString;
use std::os::fd::{FromRawFd, OwnedFd};
use crate::flags::{OpenFlags, PermissionFlags};
use crate::error::{Error, Result};
use crate::file::File;
use crate::prelude::OpenFS;
use crate::syscall;
use crate::syscall::Syscall;

impl<P> OpenFS<P> for File
where
    P: AsRef<str>,
{
    fn open(path: P, open_flags: OpenFlags, perms: PermissionFlags) -> Result<Self> {
        let c_path = CString::new(path.as_ref())
            .map_err(|_| Error::InvalidArgument)?;

        let raw_fd = syscall!(
            Syscall::Openat,
            -2,
            c_path.as_ptr(),
            open_flags.bits(),
            perms.bits()
        );
        Error::result(raw_fd)?;

        return Ok(Self {
            file: unsafe { OwnedFd::from_raw_fd(raw_fd) }
        })
    }
}