use ample::r#type::Vec;

use userspace::file::format::elf::memory_image::{MemoryImage, Region};

#[test]
fn reads_bytes_from_loaded_memory_region() {
    let bytes = [0x10, 0x20, 0x30, 0x40, 0x50];
    let image = MemoryImage::new(Vec::from([Region::new(0x5000, &bytes)]));

    assert_eq!(image.bytes(0x5001, 3), Some(&bytes[1..4]));
}

#[test]
fn reads_bytes_from_matching_region_among_multiple_regions() {
    let first = [1, 2, 3, 4];
    let second = [5, 6, 7, 8];
    let image = MemoryImage::new(Vec::from([
        Region::new(0x1000, &first),
        Region::new(0x3000, &second),
    ]));

    assert_eq!(image.bytes(0x3001, 2), Some(&second[1..3]));
}

#[test]
fn rejects_memory_image_address_outside_regions() {
    let bytes = [1, 2, 3, 4];
    let image = MemoryImage::new(Vec::from([Region::new(0x2000, &bytes)]));

    assert_eq!(image.bytes(0x1fff, 1), None);
    assert_eq!(image.bytes(0x2004, 1), None);
}

#[test]
fn does_not_join_storage_bytes_across_memory_regions() {
    let first = [1, 2];
    let second = [3, 4];
    let image = MemoryImage::new(Vec::from([
        Region::new(0x1000, &first),
        Region::new(0x1002, &second),
    ]));

    assert_eq!(image.bytes(0x1001, 2), None);
}
