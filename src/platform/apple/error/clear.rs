use crate::error::Error;

impl Error {
    pub fn clear() {
        Self::set_raw(0);
    }
}