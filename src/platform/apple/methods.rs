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
            mod id;
            mod size;
            mod time;
        }
    }
    mod io {
        mod read;
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