use crate::file::{File, SeekWhence};
use crate::flags::{OpenFlags, PermissionFlags};
use crate::prelude::*;

#[test]
fn open_file() -> Result<(), Box<dyn std::error::Error>> {
    let _ = File::open(
        "./test_files/open.txt",
        OpenFlags::ReadOnly | OpenFlags::Create | OpenFlags::Truncate,
        PermissionFlags::OwnerWrite | PermissionFlags::AllRead,
    )?;

    Ok(())
}

#[test]
fn read_write_seek_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(
        "./test_files/read_write_seek.txt",
        OpenFlags::ReadWrite | OpenFlags::Create | OpenFlags::Truncate,
        PermissionFlags::OwnerWrite | PermissionFlags::AllRead,
    )?;

    let _ = file.write("Hello, world!")?;

    let _ = file.seek(SeekWhence::StartPos(0));
    let (data, len) = file.read()?;

    assert_eq!(data, "Hello, world!");
    assert_eq!(len, 13);

    Ok(())
}

#[test]
fn seek_tell_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(
        "./test_files/seek_tell.txt",
        OpenFlags::ReadWrite | OpenFlags::Create | OpenFlags::Truncate,
        PermissionFlags::OwnerWrite | PermissionFlags::AllRead,
    )?;

    let _ = file.write("Hello, world!")?;

    let pos = file.seek(SeekWhence::CurrentPos(-3))?;

    assert_eq!(pos, file.tell()?);

    Ok(())
}

#[test]
fn truncate_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(
        "./test_files/truncate.txt",
        OpenFlags::ReadWrite | OpenFlags::Create | OpenFlags::Truncate,
        PermissionFlags::OwnerWrite | PermissionFlags::AllRead,
    )?;

    let _ = file.write("Hello, world!")?;

    let _ = file.truncate(5)?;
    let _ = file.seek(SeekWhence::StartPos(0))?;
    let (data, len) = file.read()?;

    assert_eq!(data, "Hello");
    assert_eq!(len, 5);

    Ok(())
}

#[test]
fn insert_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(
        "./test_files/insert.txt",
        OpenFlags::ReadWrite | OpenFlags::Create | OpenFlags::Truncate,
        PermissionFlags::OwnerWrite | PermissionFlags::AllRead,
    )?;

    let _ = file.write("Helloworld!")?;

    let _ = file.insert(SeekWhence::CurrentPos(-6), ", ");

    let _ = file.seek(SeekWhence::StartPos(0))?;
    let (data, len) = file.read()?;

    assert_eq!(data, "Hello, world!");
    assert_eq!(len, 13);

    Ok(())
}
