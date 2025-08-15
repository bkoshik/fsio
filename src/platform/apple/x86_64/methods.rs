mod file {
    mod fs {
        mod drop;
        mod eq;
        #[cfg(feature = "from")]
        mod from;
        mod metadata;
        mod open;
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
    mod set;
}
