pub trait Write<T>: Sized
where
    T: AsRef<str>
{
    fn write(&mut self, buf: T) -> crate::error::Result<usize>;
}