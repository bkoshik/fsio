use crate::define_bitflags;

define_bitflags!(
    pub bitflag FileTypeFlags: u32 {
        NamedPipe = 0o10_000,           // libc::S_IFIFO
        CharacterDevice = 0o20_000,     // libc::S_IFCHR
        Directory = 0o40_000,           // libc::S_IFDIR
        BlockDevice = 0o60_000,         // libc::S_IFBLK
        Regular = 0o100_000,            // libc::S_IFREG
        Symlink = 0o120_000,            // libc::S_IFLNK
        Socket = 0o140_000,             // libc::S_IFSOCK
        MaskType = 0o170_000,           // libc::S_IFMT
    }
);
