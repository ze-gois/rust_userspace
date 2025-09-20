pub trait Readable<Origin> {
    type Origin;
    fn read_from_path(path: &str, offset: usize, endianness: bool) -> Self;
    fn read_from_descriptor(file_descriptor: isize, offset: usize, endianness: bool) -> Self;
    fn read_from_pointer(ptr: *const u8, offset: usize, endianness: bool) -> Self;
}

impl<U> Readable<crate::Origin> for U
where
    U: ample::traits::Bytes<crate::Origin, crate::Origin>,
    U: Default,
{
    type Origin = crate::Origin;

    fn read_from_path(path: &str, offset: usize, endianness: bool) -> Self {
        let file_descriptor = crate::file::open(path);
        Self::read_from_descriptor(file_descriptor, offset, endianness)
    }

    fn read_from_descriptor(file_descriptor: isize, offset: usize, endianness: bool) -> Self
    where
        Self: ample::traits::Bytes<crate::Origin, crate::Origin>,
    {
        let _ = crate::file::seek(file_descriptor, offset as i64);
        let size = <Self as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
        // let bytes = [0u8; <Self as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
        use crate::memory::heap::Allocating;
        let bytes = u8::allocate(size);
        let _ = crate::target::os::syscall::read(file_descriptor, bytes, size);
        Self::from_bytes_pointer(bytes, endianness)
        // Self::default()
    }

    fn read_from_pointer(ptr: *const u8, offset: usize, endianness: bool) -> Self {
        Self::default()
    }
}

impl<A> Readable<ample::Origin> for A
where
    A: crate::traits::Ample,
    A: Default,
{
    type Origin = ample::Origin;

    fn read_from_path(path: &str, offset: usize, endianness: bool) -> Self {
        Self::default()
    }

    fn read_from_descriptor(fd: isize, offset: usize, endianness: bool) -> Self {
        Self::default()
    }

    fn read_from_pointer(ptr: *const u8, offset: usize, endianness: bool) -> Self {
        Self::default()
    }
}
