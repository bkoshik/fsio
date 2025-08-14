pub mod file {
    mod file;
    pub use file::*;

    mod file_metadata {
        mod file_type;
        pub use file_type::*;

        mod file_metadata;
        pub use file_metadata::*;
    }
    pub use file_metadata::*;
}

pub mod error {
    mod error;
    pub use error::*;

    mod result;
    pub use result::*;
}