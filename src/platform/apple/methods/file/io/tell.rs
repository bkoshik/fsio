use crate::error::*;
use crate::file::{File, SeekWhence};

impl File {
    pub fn tell(&mut self) -> Result<usize> {
        return self.seek(SeekWhence::CurrentPos(0));
    }
}
