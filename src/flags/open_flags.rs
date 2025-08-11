use nix::fcntl::OFlag;

pub const READ: OFlag = OFlag::O_RDONLY;
pub const WRITE: OFlag = OFlag::O_WRONLY;
pub const READ_WRITE: OFlag = OFlag::O_RDWR;
pub const CREATE: OFlag = OFlag::O_CREAT;
pub const EXCLUSIVE_CREATE: OFlag = OFlag::O_EXCL;
pub const NO_CONTROLLING_TERMINAL: OFlag = OFlag::O_NOCTTY;
pub const TRUNCATE: OFlag = OFlag::O_TRUNC;
pub const APPEND: OFlag = OFlag::O_APPEND;
pub const NON_BLOCKING: OFlag = OFlag::O_NONBLOCK;
pub const DATA_SYNC: OFlag = OFlag::O_DSYNC;
pub const FULL_SYNC: OFlag = OFlag::O_SYNC;
pub const CLOSE_ON_EXEC: OFlag = OFlag::O_CLOEXEC;
pub const DIRECTORY_ONLY: OFlag = OFlag::O_DIRECTORY;
pub const NO_FOLLOW_SYMLINK: OFlag = OFlag::O_NOFOLLOW;