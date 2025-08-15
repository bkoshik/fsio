use std::os::fd::OwnedFd;

#[derive(Debug)]
pub struct File {
    pub(crate) file: OwnedFd,
}
