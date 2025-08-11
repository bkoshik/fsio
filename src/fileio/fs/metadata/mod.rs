pub mod file_type;
mod time;
mod size;
mod get_metadata_fileio;

use nix::libc::stat;

pub struct FileIOMetadata {
    stat: stat
}
