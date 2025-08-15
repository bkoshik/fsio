mod platform {
    #[cfg(apple_os)]
    mod apple;
    #[cfg(apple_os)]
    pub use apple::*;

    #[cfg(linux)]
    mod linux;
    #[cfg(linux)]
    pub use linux::*;
}
pub use platform::*;

mod macros;

#[cfg(test)]
mod test;
