use crate::error::{Error, Result};
use crate::file::File;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::prelude::OpenFS;
use crate::syscall;
use crate::syscall::Syscall;
use std::ffi::CString;

impl<P> OpenFS<P> for File
where
    P: AsRef<str>,
{
    fn open(path: P, open_flags: OpenFlags, perms: PermissionFlags) -> Result<Self> {
        let c_path = CString::new(path.as_ref()).map_err(|_| Error::InvalidArgument)?;

        let fd = {
            let ret = syscall!(
                Syscall::Openat,
                -2,
                c_path.as_ptr(),
                open_flags.bits(),
                perms.bits()
            );
            Error::result(ret)?;

            ret as u64
        };

        return Ok(Self { file: fd });
    }
}
