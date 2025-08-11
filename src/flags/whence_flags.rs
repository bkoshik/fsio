use nix::unistd::Whence;

pub const START_POS: Whence = Whence::SeekSet;
pub const CURRENT_POS: Whence = Whence::SeekCur;
pub const END_POS: Whence = Whence::SeekEnd;