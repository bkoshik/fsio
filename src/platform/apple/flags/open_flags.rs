use crate::define_bitflags;
use crate::flags::prefix::OpenPrefix;

define_bitflags!(
    let prefix = OpenPrefix;
    pub struct OpenFlags: usize {
        ReadOnly,
        WriteOnly,
        ReadWrite,
        AccessMode,
        NonBlocking,
        Append,
        SharedLock,
        ExclusiveLock,
        Async,
        Sync,
        NoFollowSymlink,
        Create,
        Truncate,
        Exclusive,
        OnlyEvents,
        NoControlTTY,
        DirectoryOnly,
        AllowSymlink,
        SyncData,
        CloseOnExec,
        NoFollowAny,
        ExecuteOnly,
        SearchOnly,
    }
);