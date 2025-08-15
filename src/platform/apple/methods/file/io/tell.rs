use crate::file::{File, SeekWhence};
use crate::error::*;

impl File {
    pub fn tell(&mut self) -> Result<usize> {
        return self.seek(SeekWhence::CurrentPos(0))
    }
}