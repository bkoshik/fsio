use crate::define_bitflags;

define_bitflags!(
    
    pub bitflag  OpenFlags: usize {
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
);