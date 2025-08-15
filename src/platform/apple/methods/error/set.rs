use crate::error::Error;

impl Error {
    pub fn set(self) {
        Self::set_raw(self as i32);
    }

    pub fn set_raw(err: i32) {
        unsafe { *libc::__error() = err };
    }
}
