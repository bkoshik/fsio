#[cfg(target_arch = "aarch64")]
mod aarch64 {
    mod methods;
    mod types;

    pub mod prelude;
    pub mod syscall;

    pub mod api;
    pub use api::*;
}
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    mod methods;
    mod types;

    pub mod prelude;
    pub mod syscall;

    pub mod api;
    pub use api::*;
}
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
