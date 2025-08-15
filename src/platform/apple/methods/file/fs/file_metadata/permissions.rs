use crate::file::{FileMetadata, Permissions};
use crate::flags::PermissionFlags;

impl FileMetadata {
    pub fn permissions(&self) -> Permissions {
         return Permissions::from_raw(self.metadata.st_mode)
    }

    pub fn permission_flags(&self) -> PermissionFlags {
        return PermissionFlags::from_bits_truncate(self.metadata.st_mode as u32 & PermissionFlags::MaskType.bits())
    }
}