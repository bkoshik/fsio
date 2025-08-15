pub trait Write<B>: Sized
where
    B: AsRef<str>
{
    fn write(&mut self, buf: B) -> crate::error::Result<usize>;
}