mod flags;
mod fs;

use std::os::fd::OwnedFd;

pub struct FileIO {
    file: OwnedFd
}