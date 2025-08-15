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
        mod read;
        mod seek;
        mod truncate;
        mod write;
    }
    #[cfg(feature = "extra")]
    mod extra {
        mod io {
            mod insert;
            mod tell;
        }
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
