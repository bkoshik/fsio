mod flags;
mod fs;
mod io;

use std::os::fd::OwnedFd;

pub struct FileIO {
    file: OwnedFd
}