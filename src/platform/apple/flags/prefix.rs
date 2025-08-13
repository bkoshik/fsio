#[repr(usize)]
pub enum OpenPrefix {
    ReadOnly = 0x0,                     // libc::O_RDONLY
    WriteOnly = 0x1,                    // libc::O_WRONLY
    ReadWrite = 0x2,                    // libc::O_RDWR
    AccessMode = 0x3,                   // libc::O_ACCMODE
    NonBlocking = 0x4,                  // libc::O_NONBLOCK
    Append = 0x8,                       // libc::O_APPEND
    SharedLock = 0x10,                  // libc::O_SHLOCK
    ExclusiveLock = 0x20,               // libc::O_EXLOCK
    Async = 0x40,                       // libc::O_ASYNC
    Sync = 0x80,                        // libc::O_SYNC
    NoFollowSymlink = 0x100,            // libc::O_NOFOLLOW
    Create = 0x200,                     // libc::O_CREAT
    Truncate = 0x400,                   // libc::O_TRUNC
    Exclusive = 0x800,                  // libc::O_EXCL
    OnlyEvents = 0x8000,                // libc::O_EVTONLY
    NoControlTTY = 0x20_000,            // libc::O_NOCTTY
    DirectoryOnly = 0x100_000,          // libc::O_DIRECTORY
    AllowSymlink = 0x200_000,           // libc::O_SYMLINK
    SyncData = 0x400_000,               // libc::O_DSYNC
    CloseOnExec = 0x1_000_000,          // libc::O_CLOEXEC
    NoFollowAny = 0x20_000_000,         // libc::O_NOFOLLOW_ANY
    ExecuteOnly = 0x40_000_000,         // libc::O_EXEC
    SearchOnly = 0x41_000_000,          // libc::O_SEARCH
}

#[repr(usize)]
pub enum FileTypePrefix {
    NamedPipe = 0o10_000,
    CharacterDevice = 0o20_000,
    Directory = 0o40_000,
    BlockDevice = 0o60_000,
    Regular = 0o100_000,
    Symlink = 0o120_000,
    Socket = 0o140_000,
    MaskType = 0o170_000
}

#[repr(usize)]
pub enum PermissionsPrefix {
    AllRWX = 0o777,
    AllRead = 0o444,
    AllWrite = 0o222,
    AllExec = 0o111,

    OwnerRWX = 0o700,
    OwnerRead = 0o400,
    OwnerWrite = 0o200,
    OwnerExec = 0o100,

    GroupRWX = 0o070,
    GroupRead = 0o040,
    GroupWrite = 0o020,
    GroupExec = 0o010,

    OtherRWX = 0o007,
    OtherRead = 0o004,
    OtherWrite = 0o002,
    OtherExec = 0o001,
}