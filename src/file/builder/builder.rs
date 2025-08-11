use nix::{
    fcntl::OFlag,
    sys::stat::Mode,
    libc::mode_t
};
use crate::file::FileBuilder;

impl FileBuilder {
    pub fn new() -> Self {
        return Self {
            flags: OFlag::empty(),
            permissions: Mode::from_bits(0o755).unwrap()
        }
    }

    pub fn read(mut self) -> Self {
        self.flags |= OFlag::O_RDONLY;
        return self
    }

    pub fn write(mut self) -> Self {
        self.flags |= OFlag::O_WRONLY;
        return self
    }

    pub fn read_write(mut self) -> Self {
        self.flags |= OFlag::O_RDWR;
        return self
    }

    pub fn create(mut self) -> Self {
        self.flags |= OFlag::O_CREAT;
        return self
    }

    pub fn truncate(mut self) -> Self {
        self.flags |= OFlag::O_TRUNC;
        return self
    }

    pub fn append(mut self) -> Self {
        self.flags |= OFlag::O_APPEND;
        return self
    }

    pub fn permissions(mut self, perms: u32) -> Self {
        self.permissions = Mode::from_bits(perms as mode_t).unwrap();
        return self
    }
}