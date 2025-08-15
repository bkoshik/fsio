mod file {
    mod fs {
        mod open;
        mod drop;
        #[cfg(feature = "from")]
        mod from;
        mod eq;
        mod metadata;
        mod file_metadata {
            mod file_type;
            mod id;
            mod inode;
            mod permissions;
            mod size;
            mod time;
        }
    }
    mod io {
        mod insert;
        mod read;
        mod seek;
        mod tell;
        mod truncate;
        mod write;
    }
}

mod error {
    mod clear;
    mod display;
    mod from_raw;
    mod last;
    mod result;
    mod set;
}
