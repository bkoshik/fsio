mod file {
    mod fs {
        mod open;
        mod metadata;
        mod as_fd;
        mod eq;
        mod file_metadata {
            mod file_type;
            mod permissions;
            mod inode;
        }
    }
}

mod error {
    mod from_raw;
    mod last;
    mod display;
    mod set;
    mod clear;
    mod result;
}