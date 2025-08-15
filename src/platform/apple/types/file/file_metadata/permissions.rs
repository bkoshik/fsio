use libc::mode_t;
use crate::flags::PermissionFlags;

pub struct Permissions {
    pub owner: Access,
    pub group: Access,
    pub other: Access,
}

pub struct Access {
    pub read: bool,
    pub write: bool,
    pub exec: bool,
}

impl Permissions {
    pub fn from_raw(raw_mode: usize) -> Self {
        return Self {
            owner: Access {
                read: raw_mode & PermissionFlags::OwnerRead.bits() != 0,
                write: raw_mode & PermissionFlags::OwnerWrite.bits() != 0,
                exec: raw_mode & PermissionFlags::OwnerExec.bits() != 0,
            },
            group: Access {
                read: raw_mode & PermissionFlags::GroupRead.bits() != 0,
                write: raw_mode & PermissionFlags::GroupWrite.bits() != 0,
                exec: raw_mode & PermissionFlags::GroupExec.bits() != 0,
            },
            other: Access {
                read: raw_mode & PermissionFlags::OtherRead.bits() != 0,
                write: raw_mode & PermissionFlags::OtherWrite.bits() != 0,
                exec: raw_mode & PermissionFlags::OtherExec.bits() != 0,
            },
        }
    }
}