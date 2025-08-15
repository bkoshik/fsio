use crate::file::SeekWhence;

pub trait Insert<B>: Sized
where
    B: AsRef<str>,
{
    fn insert(&mut self, whence: SeekWhence, buf: B) -> crate::error::Result<u64>;
}
