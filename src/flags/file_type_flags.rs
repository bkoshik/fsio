use nix::sys::stat::SFlag;

pub const REGULAR: SFlag = SFlag::S_IFREG;
pub const SYMLINK: SFlag = SFlag::S_IFLNK;
pub const DIRECTORY: SFlag = SFlag::S_IFDIR;
pub const BLOCK: SFlag = SFlag::S_IFBLK;
pub const CHAR_DEVICE: SFlag = SFlag::S_IFCHR;
pub const SOCKET: SFlag = SFlag::S_IFSOCK;
pub const MASK_TYPE: SFlag = SFlag::S_IFMT;
pub const NAMED_PIPE: SFlag = SFlag::S_IFIFO;