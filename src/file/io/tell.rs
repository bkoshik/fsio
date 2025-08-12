use crate::{
    errors::*,
    flags::whence_flags::CURRENT_POS,
    file::{
        File,
    }
};

impl File {
    pub fn tell(&mut self) -> Result<usize> {
        return Ok(self.seek(0, CURRENT_POS)? as usize
       );
    }
}