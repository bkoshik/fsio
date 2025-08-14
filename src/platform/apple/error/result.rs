use std::ffi::c_void;
use crate::error::*;

// This code snippet is adapted from the `nix` crate (https://github.com/nix-rust/nix)
impl Error {
    #[inline]
    pub fn result<S: ErrorSentinel + PartialEq<S>>(value: S) -> Result<S> {
        if value == S::sentinel() {
            Err(Self::last())
        } else {
            Ok(value)
        }
    }
}

pub trait ErrorSentinel: Sized {
    fn sentinel() -> Self;
}

impl ErrorSentinel for isize {
    fn sentinel() -> Self {
        return -1
    }
}

impl ErrorSentinel for i32 {
    fn sentinel() -> Self {
        return -1
    }
}

impl ErrorSentinel for i64 {
    fn sentinel() -> Self {
        return -1
    }
}

impl ErrorSentinel for *mut c_void {
    fn sentinel() -> Self {
        return -1isize as *mut c_void
    }
}

impl ErrorSentinel for libc::sighandler_t {
    fn sentinel() -> Self {
        return !0
    }
}