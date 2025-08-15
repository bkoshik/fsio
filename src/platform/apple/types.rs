pub(crate) mod file {
    pub(crate) mod file;
    pub(crate) mod seek_whence;
    pub(crate) mod file_metadata {
        pub(crate) mod file_metadata;
        pub(crate) mod file_type;
        pub(crate) mod permissions;
    }
}

pub(crate) mod error {
    pub(crate) mod error;
    pub(crate) mod result;
}

pub(crate) mod flags {
    pub(crate) mod open_flags;
    pub(crate) mod file_type_flags;
    pub(crate) mod permission_flags;
}