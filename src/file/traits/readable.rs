pub trait Readable<Origin>
where
    Self: Copy,
    Self: Sized,
    Self: ample::traits::Bytes<Self::Origin, Self::Origin>,
    Self: crate::memory::heap::Allocating<Self>,
{
    type Origin = Origin;
    fn read_from_file_path(
        file_path: &str,
        offset: usize,
        endianness: bool,
    ) -> (Self, isize, usize) {
        let file_descriptor = crate::file::open(file_path);
        let (value, size) = Self::read_from_file_descriptor(file_descriptor, offset, endianness);
        (value, file_descriptor, size)
    }

    fn read_from_file_descriptor(
        file_descriptor: isize,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize) {
        let _ = crate::file::seek(file_descriptor, offset as i64);
        let size = <Self as ample::traits::Bytes<Self::Origin, Self::Origin>>::BYTES_SIZE;
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
        let value = Self::from_bytes_pointer(unsafe { bytes_pointer.add(offset) }, endianness);
        (value, Self::BYTES_SIZE + offset)
    }

    fn read_from_file_path_offsets(
        file_path: &str,
        offsets: &[usize],
        endianness: bool,
    ) -> (&'static mut [Self], isize) {
        let file_descriptor = crate::file::open(file_path);
        Self::read_from_file_descriptor_offsets(file_descriptor, offsets, endianness)
    }

    fn read_from_file_descriptor_offsets(
        file_descriptor: isize,
        offsets: &[usize],
        endianness: bool,
    ) -> (&'static mut [Self], isize) {
        use crate::memory::heap::Allocating;

        let bytes_pointer = u8::allocate(Self::BYTES_SIZE);
        let values = Self::allocate_slice(offsets.len());
        for (o, offset) in offsets.iter().enumerate() {
            let _ = crate::file::seek(file_descriptor, *offset as i64);
            let _ =
                crate::target::os::syscall::read(file_descriptor, bytes_pointer, Self::BYTES_SIZE);
            values[o] = Self::from_bytes_pointer(bytes_pointer, endianness);
        }
        (values, file_descriptor)
    }

    fn read_from_pointer_offsets(
        bytes_pointer: *const u8,
        offsets: &[usize],
        endianness: bool,
    ) -> &'static mut [Self] {
        let values = Self::allocate_slice(offsets.len());
        for (o, offset) in offsets.iter().enumerate() {
            values[o] = Self::from_bytes_pointer(unsafe { bytes_pointer.add(*offset) }, endianness);
        }
        values
    }
}

impl<U> Readable<crate::Origin> for U
where
    Self: Copy,
    Self: ample::traits::BytesDefault<crate::Origin>,
    Self: crate::memory::heap::Allocating<Self>,
{
    type Origin = crate::Origin;
}

// impl<A> Readable<ample::Origin> for A
// where
//     Self: ample::traits::Bytes<ample::Origin, ample::Origin>,
//     Self: crate::memory::heap::Allocating<Self>,
// {
//     type Origin = ample::Origin;
// }
