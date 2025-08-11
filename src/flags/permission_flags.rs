use nix::sys::stat::Mode;

pub const OWNER_READ: Mode  = Mode::S_IRUSR;
pub const OWNER_WRITE: Mode = Mode::S_IWUSR;
pub const OWNER_EXEC: Mode  = Mode::S_IXUSR;

pub const GROUP_READ: Mode  = Mode::S_IRGRP;
pub const GROUP_WRITE: Mode = Mode::S_IWGRP;
pub const GROUP_EXEC: Mode  = Mode::S_IXGRP;

pub const OTHERS_READ: Mode  = Mode::S_IROTH;
pub const OTHERS_WRITE: Mode = Mode::S_IWOTH;
pub const OTHERS_EXEC: Mode  = Mode::S_IXOTH;

pub const ALL: Mode = Mode::all();
pub const EMPTY: Mode = Mode::empty();