mod open_flags;
mod file_type_flags;
mod permission_flags;

pub use open_flags::*;
pub use file_type_flags::*;
pub use permission_flags::*;

fn main() {
    OpenFlags::from_bits_truncate(0x10100);
}