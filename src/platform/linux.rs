mod aarch64 {
    pub mod prelude;
    pub mod syscall;

    pub mod api;
    pub use api::*;
}

mod x86_64 {
    pub mod prelude;
    pub mod syscall;

    pub mod api;
    pub use api::*;
}