use crate::file::{File, SeekWhence};
use crate::prelude::{Insert, Write};
use crate::error::*;

impl<B> Insert<B> for File
where
    B: AsRef<str>,
{
    fn insert(&mut self, whence: SeekWhence, buf: B) -> Result<usize> {
        let original_pos = self.tell()?;

        let target_pos = self.seek(whence)?;
        let (data_from_offset, _) = self.read()?;

        let _ = self.seek(SeekWhence::StartPos(target_pos))?;
        let _ = self.write(buf.as_ref())?;
        let _ = self.write(data_from_offset)?;

        return self.seek(SeekWhence::StartPos(original_pos));
    }
}