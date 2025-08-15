use crate::error::{Error, Result};
use crate::file::File;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::prelude::OpenFS;
use crate::syscall::*;
use std::ffi::CString;

impl<P> OpenFS<P> for File
where
    P: AsRef<str>,
{
    fn open(path: P, open_flags: OpenFlags, perms: PermissionFlags) -> Result<Self> {
        let c_path = CString::new(path.as_ref()).map_err(|_| Error::InvalidArgument)?;

        let fd = {
            let mut args = [0i64; 6];
            args[0] = -2;
            args[1] = c_path.as_ptr() as i64;
            args[2] = open_flags.bits() as i64;
            args[3] = perms.bits() as i64;

            let ret = syscall(Syscall::Openat, &args)?;

            ret as u64
        };

        return Ok(Self { file: fd });
    }
}
