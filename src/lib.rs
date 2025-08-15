mod platform {
    #[cfg(apple_os)]
    mod apple;
    #[cfg(apple_os)]
    pub use apple::*;
}
pub use platform::*;

mod macros;