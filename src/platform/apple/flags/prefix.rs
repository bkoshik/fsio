#[repr(usize)]
pub enum OpenPrefix {
    ReadOnly = 0,                   // libc::O_RDONLY = 0
    WriteOnly = 1,                  // libc::O_WRONLY = 1
    ReadWrite = 2,                  // libc::O_RDWR = 2
    AccessMode = 3,                 // libc::O_ACCMODE = 3
    NonBlocking = 4,                // libc::O_NONBLOCK = 4
    Append = 8,                     // libc::O_APPEND = 8
    SharedLock = 10,                // libc::O_SHLOCK = 10
    ExclusiveLock = 32,             // libc::O_EXLOCK = 32
    Async = 64,                     // libc::O_ASYNC = 64
    Sync = 128,                     // libc::O_SYNC = 128
    NoFollowSymlink = 256,          // libc::O_NOFOLLOW = 256
    Create = 512,                   // libc::O_CREAT = 512
    Truncate = 1024,                // libc::O_TRUNC = 1024
    Exclusive = 2048,               // libc::O_EXCL = 2048
    OnlyEvents = 32_768,            // libc::O_EVTONLY = 32_768
    NoControlTTY = 131_072,         // libc::O_NOCTTY = 131_072
    DirectoryOnly = 1_048_576,      // libc::O_DIRECTORY = 1_048_576
    AllowSymlink = 2_097_152,       // libc::O_SYMLINK = 2_097_152
    SyncData = 4_194_304,           // libc::O_DSYNC = 4_194_304
    CloseOnExec = 16_777_216,       // libc::O_CLOEXEC = 16_777_216
    NoFollowAny = 536_870_912,      // libc::O_NOFOLLOW_ANY = 536_870_912
    ExecuteOnly = 1_073_741_824,    // libc::O_EXEC = 1_073_741_824
    SearchOnly = 1_090_519_040,     // libc::O_SEARCH = 1_090_519_040
}