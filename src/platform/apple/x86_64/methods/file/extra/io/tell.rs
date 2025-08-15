use crate::error::*;
use crate::file::{File, SeekWhence};

impl File {
    pub fn tell(&mut self) -> Result<u64> {
        return self.seek(SeekWhence::CurrentPos(0));
    }
}
