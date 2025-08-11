mod fs;
mod io;

pub mod prelude;
pub mod builder;

use std::os::fd::OwnedFd;

pub struct FileIO {
    file: OwnedFd
}