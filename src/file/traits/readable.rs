pub trait Readable<Origin>
where
    Self: Sized,
    Self: ample::traits::Bytes<Origin, Origin>,
    Self: crate::memory::heap::Allocating<Self>,
{
    type Origin = Origin;
    fn read_from_path(file_path: &str, offset: usize, endianness: bool) -> (Self, usize) {
        let file_descriptor = crate::file::open(file_path);
        Self::read_from_descriptor(file_descriptor, offset, endianness)
    }

    fn read_from_descriptor(
        file_descriptor: isize,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize) {
        let _ = crate::file::seek(file_descriptor, offset as i64);
        let size = <Self as ample::traits::Bytes<Origin, Origin>>::BYTES_SIZE;
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

    fn read_from_path_offsets(
        file_path: &str,
        offsets: &[usize],
        endianness: bool,
    ) -> &'static [Self] {
        let file_descriptor = crate::file::open(file_path);
        Self::read_from_descriptor_offsets(file_descriptor, offsets, endianness)
    }

    fn read_from_descriptor_offsets(
        file_descriptor: isize,
        offsets: &[usize],
        endianness: bool,
    ) -> &'static mut [Self] {
        use crate::memory::heap::Allocating;

        let bytes = u8::allocate(Self::BYTES_SIZE);
        let values = Self::allocate_slice(offsets.len());
        for (o, offset) in offsets.iter().enumerate() {
            let _ = crate::file::seek(file_descriptor, *offset as i64);
            let _ = crate::target::os::syscall::read(file_descriptor, bytes, Self::BYTES_SIZE);
            let (value, _) = Self::read_from_pointer(bytes, 0, endianness);
            values[o] = value;
        }
        values
    }

    fn read_from_pointer_offsets(
        bytes_pointer: *const u8,
        offsets: &[usize],
        endianness: bool,
    ) -> &'static mut [Self] {
        let values = Self::allocate_slice(offsets.len());
        for (o, offset) in offsets.iter().enumerate() {
            let (value, _) =
                Self::read_from_pointer(unsafe { bytes_pointer.add(*offset) }, 0, endianness);
            values[o] = value;
        }
        values
    }
}

impl<U> Readable<crate::Origin> for U
where
    Self: Sized,
    Self: Default,
    Self: ample::traits::Bytes<crate::Origin, crate::Origin>,
    Self: crate::memory::heap::Allocating<Self>,
{
    type Origin = crate::Origin;
}

impl<A> Readable<ample::Origin> for A
where
    Self: Sized,
    Self: Default,
    Self: ample::traits::Bytes<ample::Origin, ample::Origin>,
    Self: crate::memory::heap::Allocating<Self>,
{
    type Origin = ample::Origin;
}
