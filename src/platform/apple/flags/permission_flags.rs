use enum_flags::EnumFlags;

#[derive(EnumFlags)]
#[repr(usize)]
pub enum PermissionsPrefix {
    AllRWX = 0o777,
    AllRead = 0o444,
    AllWrite = 0o222,
    AllExec = 0o111,

    OwnerRWX = 0o700,
    OwnerRead = 0o400,
    OwnerWrite = 0o200,
    OwnerExec = 0o100,

    GroupRWX = 0o070,
    GroupRead = 0o040,
    GroupWrite = 0o020,
    GroupExec = 0o010,

    OtherRWX = 0o007,
    OtherRead = 0o004,
    OtherWrite = 0o002,
    OtherExec = 0o001,
}