use crate::file::File;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::prelude::*;

#[test]
fn open_file() -> Result<(), Box<dyn std::error::Error>> {
    let _ = File::open(
        "./test.txt",
        OpenFlags::ReadOnly | OpenFlags::Create | OpenFlags::Truncate,
        PermissionFlags::empty(),
    )?;

    Ok(())
}