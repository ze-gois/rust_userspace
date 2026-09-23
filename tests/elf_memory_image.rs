use ample::r#type::Vec;

use userspace::file::format::elf::memory_image::{
    MemoryImage, MemoryImageWriter, Region, RegionWriter,
};

#[test]
fn reads_bytes_from_loaded_memory_region() {
    let bytes = [0x10, 0x20, 0x30, 0x40, 0x50];
    let mut regions = Vec::new();
    regions.push(Region::new(0x5000, &bytes));
    let image = MemoryImage::new(regions);

    assert_eq!(image.bytes(0x5001, 3), Some(&bytes[1..4]));
}

#[test]
fn reads_bytes_from_matching_region_among_multiple_regions() {
    let first = [1, 2, 3, 4];
    let second = [5, 6, 7, 8];
    let mut regions = Vec::new();
    regions.push(Region::new(0x1000, &first));
    regions.push(Region::new(0x3000, &second));
    let image = MemoryImage::new(regions);

    assert_eq!(image.bytes(0x3001, 2), Some(&second[1..3]));
}

#[test]
fn rejects_memory_image_address_outside_regions() {
    let bytes = [1, 2, 3, 4];
    let mut regions = Vec::new();
    regions.push(Region::new(0x2000, &bytes));
    let image = MemoryImage::new(regions);

    assert_eq!(image.bytes(0x1fff, 1), None);
    assert_eq!(image.bytes(0x2004, 1), None);
}

#[test]
fn does_not_join_storage_bytes_across_memory_regions() {
    let first = [1, 2];
    let second = [3, 4];
    let mut regions = Vec::new();
    regions.push(Region::new(0x1000, &first));
    regions.push(Region::new(0x1002, &second));
    let image = MemoryImage::new(regions);

    assert_eq!(image.bytes(0x1001, 2), None);
}


#[test]
fn writes_bytes_to_loaded_memory_region() {
    let mut bytes = [0u8; 6];
    let mut regions = Vec::new();
    regions.push(RegionWriter::new(0x4000, &mut bytes));
    let mut image = MemoryImageWriter::new(regions);

    assert!(image.write(0x4001, &[0xaa, 0xbb, 0xcc]));
    assert_eq!(bytes, [0x00, 0xaa, 0xbb, 0xcc, 0x00, 0x00]);
}

#[test]
fn rejects_write_outside_loaded_memory_regions() {
    let mut bytes = [0u8; 4];
    let mut regions = Vec::new();
    regions.push(RegionWriter::new(0x5000, &mut bytes));
    let mut image = MemoryImageWriter::new(regions);

    assert!(!image.write(0x4fff, &[1]));
    assert!(!image.write(0x5004, &[1]));
    assert_eq!(bytes, [0; 4]);
}

#[test]
fn does_not_join_write_across_memory_regions() {
    let mut first = [0u8; 2];
    let mut second = [0u8; 2];
    let mut regions = Vec::new();
    regions.push(RegionWriter::new(0x1000, &mut first));
    regions.push(RegionWriter::new(0x1002, &mut second));
    let mut image = MemoryImageWriter::new(regions);

    assert!(!image.write(0x1001, &[1, 2]));
    assert_eq!(first, [0; 2]);
    assert_eq!(second, [0; 2]);
}
