pub mod builder;
pub mod open;

use nix::{
    fcntl::OFlag,
    sys::stat::Mode
};

pub struct FileIOBuilder {
    flags: OFlag,
    permissions: Mode
}