use crate::error::Error;

impl Error {
    pub fn last() -> Self {
        return Self::from_raw(Self::last_raw())
    }

    pub fn last_raw() -> i32 {
        return unsafe { *libc::__error() as i32 }
    }
}