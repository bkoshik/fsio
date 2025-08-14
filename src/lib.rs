mod platform {
    #[cfg(apple_os)]
    mod apple {
        pub mod file {
            mod fs {
                mod file_metadata {
                    mod file_type;
                }
                mod open;
                mod metadata;
                mod as_fd;
            }
            mod types;

            pub use types::*;
        }
        pub mod error {
            mod types;
            mod from_raw;
            mod last;
            mod set;
            mod clear;
            mod display;
            mod result;

            pub use types::*;
        }
        pub mod flags {
            mod open_flags;
            mod permission_flags;
            mod file_type_flags;

            pub use open_flags::*;
            pub use permission_flags::*;
            pub use file_type_flags::*;
        }
    }
    #[cfg(apple_os)]
    pub use apple::*;
}
pub use platform::*;

mod macros;