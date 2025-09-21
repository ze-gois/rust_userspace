pub trait Readable<Origin> {
    type Origin;

    fn read_from_path(file_path: &str, offset: usize, endianness: bool) -> (Self, usize)
    where
        Self: Sized;

    fn read_from_descriptor(
        file_descriptor: isize,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize)
    where
        Self: Sized;

    fn read_from_pointer(
        bytes_pointer: *const u8,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize)
    where
        Self: Sized;
}

impl<U> Readable<crate::Origin> for U
where
    U: ample::traits::Bytes<crate::Origin, crate::Origin>,
    U: Default,
{
    type Origin = crate::Origin;

    fn read_from_path(file_path: &str, offset: usize, endianness: bool) -> (Self, usize) {
        let file_descriptor = crate::file::open(file_path);
        Self::read_from_descriptor(file_descriptor, offset, endianness)
    }

    fn read_from_descriptor(
        file_descriptor: isize,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize)
    where
        Self: ample::traits::Bytes<crate::Origin, crate::Origin>,
    {
        let _ = crate::file::seek(file_descriptor, offset as i64);
        let size = <Self as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
        use crate::memory::heap::Allocating;
        let bytes = u8::allocate(size);
        let _ = crate::target::os::syscall::read(file_descriptor, bytes, size);
        Self::read_from_pointer(bytes, 0, endianness)
    }

    fn read_from_pointer(
        bytes_pointer: *const u8,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize) {
        (
            Self::from_bytes_pointer(unsafe { bytes_pointer.add(offset) }, endianness),
            Self::BYTES_SIZE + offset,
        )
    }
}

impl<A> Readable<ample::Origin> for A
where
    A: ample::traits::Bytes<ample::Origin, ample::Origin>,
    A: Default,
{
    type Origin = ample::Origin;

    fn read_from_path(file_path: &str, offset: usize, endianness: bool) -> (Self, usize) {
        let file_descriptor = crate::file::open(file_path);
        Self::read_from_descriptor(file_descriptor, offset, endianness)
    }

    fn read_from_descriptor(
        file_descriptor: isize,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize)
    where
        Self: ample::traits::Bytes<ample::Origin, ample::Origin>,
    {
        let _ = crate::file::seek(file_descriptor, offset as i64);
        let size = <Self as ample::traits::Bytes<ample::Origin, ample::Origin>>::BYTES_SIZE;
        use crate::memory::heap::Allocating;
        let bytes = u8::allocate(size);
        let _ = crate::target::os::syscall::read(file_descriptor, bytes, size);
        Self::read_from_pointer(bytes, 0, endianness)
    }

    fn read_from_pointer(
        bytes_pointer: *const u8,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize) {
        (
            Self::from_bytes_pointer(unsafe { bytes_pointer.add(offset) }, endianness),
            Self::BYTES_SIZE + offset,
        )
    }
}
