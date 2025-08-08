mod fs;
mod io;

pub mod flags;
pub mod prelude;

use std::os::fd::OwnedFd;

pub struct FileIO {
    file: OwnedFd
}