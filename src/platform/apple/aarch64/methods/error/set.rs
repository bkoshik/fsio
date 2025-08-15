use crate::error::Error;

impl Error {
    pub fn set(self) {
        Self::set_raw(self as i64);
    }

    pub fn set_raw(err: i64) {
        unsafe { *libc::__error() = err as i32 };
    }
}
