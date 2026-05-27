use crate::Result;

pub trait TryRead<'a>
where
    Self: Sized,
{
    fn try_read(data: &'a [u8]) -> Result<Self>;
}

pub trait TryWrite {
    fn try_write(&self, data: &mut [u8]) -> Result<usize>;
}
