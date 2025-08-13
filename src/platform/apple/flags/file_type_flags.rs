use crate::define_bitflags;
use crate::flags::prefix::FileTypePrefix;

define_bitflags!(
    let prefix = FileTypePrefix;
    pub struct FileTypeFlags: usize {
        NamedPipe,
        CharacterDevice,
        Directory,
        BlockDevice,
        Regular,
        Symlink,
        Socket,
        MaskType,
    }
);