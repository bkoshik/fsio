use enum_flags::EnumFlags;

#[derive(EnumFlags)]
#[repr(usize)]
pub enum FileTypePrefix {
    NamedPipe = 0o10_000,
    CharacterDevice = 0o20_000,
    Directory = 0o40_000,
    BlockDevice = 0o60_000,
    Regular = 0o100_000,
    Symlink = 0o120_000,
    Socket = 0o140_000,
    MaskType = 0o170_000
}