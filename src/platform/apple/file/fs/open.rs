use std::fs::File;
use std::io::SeekFrom;
use std::path::Path;
use crate::flags::{OpenFlags, PermissionFlags};

pub trait OpenFS<T>: Sized
where
    T: AsRef<Path>,
{
    fn open(path: T, open_flags: OpenFlags, perms: PermissionFlags);
}