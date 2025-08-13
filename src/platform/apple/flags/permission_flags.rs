use crate::define_bitflags;

define_bitflags!(
    pub bitflag PermissionFlags: usize {
        AllRWX = 0o777,
        AllRead = 0o444,
        AllWrite = 0o222,
        AllExec = 0o111,

        OwnerRWX = 0o700,       // libc::S_IRWXU
        OwnerRead = 0o400,      // libc::S_IRUSR | libc::S_IREAD
        OwnerWrite = 0o200,     // libc::S_IWUSR | libc::S_IWRITE
        OwnerExec = 0o100,      // libc::S_IXUSR | libc::S_IEXEC

        GroupRWX = 0o070,       // libc::S_IRWXG
        GroupRead = 0o040,      // libc::S_IRGRP
        GroupWrite = 0o020,     // libc::S_IWGRP
        GroupExec = 0o010,      // libc::S_IXGRP

        OtherRWX = 0o007,       // libc::S_IRWXO
        OtherRead = 0o004,      // libc::S_IROTH
        OtherWrite = 0o002,     // libc::S_IWOTH
        OtherExec = 0o001,      // libc::S_IXOTH
    }
);