use crate::memory::heap::Allocator;

pub trait Readable<Origin>
where
    Self: Copy,
    Self: Sized,
    Self: ample::traits::Bytes<Self::Origin, Self::Origin>,
{
    type Origin = Origin;
    fn read_from_path(path: &str, offset: usize, endianness: bool) -> (Self, isize, usize) {
        let file_descriptor = crate::file::open(path);
        let (value, size) = Self::read_from_file_descriptor(file_descriptor, offset, endianness);
        (value, file_descriptor, size)
    }

    fn read_from_file_descriptor(
        file_descriptor: isize,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize) {
        let _ = crate::file::seek(file_descriptor, offset as i64);
        let size = <Self as ample::traits::Bytes<Self::Origin, Self::Origin>>::REPRESENTATION_SIZE;
        let bytes = Allocator::allocate::<u8>(size);
        if bytes.is_null() {
            panic!("failed to allocate representation buffer");
        }
        let _ = crate::target::os::syscall::read(file_descriptor, bytes, size);
        Self::read_from_pointer(bytes, 0, endianness)
    }

    fn read_from_pointer(
        bytes_pointer: *const u8,
        offset: usize,
        endianness: bool,
    ) -> (Self, usize) {
        let value = Self::from_bytes_pointer(unsafe { bytes_pointer.add(offset) }, endianness);
        (value, Self::REPRESENTATION_SIZE + offset)
    }

    fn read_from_path_offsets(
        path: &str,
        offsets: &[usize],
        endianness: bool,
    ) -> (&'static mut [Self], isize) {
        let file_descriptor = crate::file::open(path);
        Self::read_from_file_descriptor_offsets(file_descriptor, offsets, endianness)
    }

    fn read_from_file_descriptor_offsets(
        file_descriptor: isize,
        offsets: &[usize],
        endianness: bool,
    ) -> (&'static mut [Self], isize) {
        let bytes_pointer = Allocator::allocate::<u8>(Self::REPRESENTATION_SIZE);
        if bytes_pointer.is_null() {
            panic!("failed to allocate representation buffer");
        }

        let values_pointer = Allocator::allocate::<Self>(offsets.len());
        if values_pointer.is_null() && !offsets.is_empty() {
            panic!("failed to allocate value buffer");
        }
        let values = unsafe { core::slice::from_raw_parts_mut(values_pointer, offsets.len()) };
        for (o, offset) in offsets.iter().enumerate() {
            let _ = crate::file::seek(file_descriptor, *offset as i64);
            let _ =
                crate::target::os::syscall::read(file_descriptor, bytes_pointer, Self::REPRESENTATION_SIZE);
            values[o] = Self::from_bytes_pointer(bytes_pointer, endianness);
        }
        (values, file_descriptor)
    }

    fn read_from_pointer_offsets(
        bytes_pointer: *const u8,
        offsets: &[usize],
        endianness: bool,
    ) -> &'static mut [Self] {
        let values_pointer = Allocator::allocate::<Self>(offsets.len());
        if values_pointer.is_null() && !offsets.is_empty() {
            panic!("failed to allocate value buffer");
        }
        let values = unsafe { core::slice::from_raw_parts_mut(values_pointer, offsets.len()) };
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
{
    type Origin = crate::Origin;
}

// impl<A> Readable<ample::Origin> for A
// where
//     Self: ample::traits::Bytes<ample::Origin, ample::Origin>,
//     // {
//     type Origin = ample::Origin;
// }
