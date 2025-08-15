use crate::flags::{OpenFlags, PermissionFlags};

pub trait OpenFS<P>: Sized
where
    P: AsRef<str>,
{
    fn open(path: P, open_flags: OpenFlags, perms: PermissionFlags) -> crate::error::Result<Self>;
}
