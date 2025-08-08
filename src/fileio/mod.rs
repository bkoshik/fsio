mod flags;
mod fs;
mod io;
pub mod prelude;

use std::os::fd::OwnedFd;

pub struct FileIO {
    file: OwnedFd
}