//! Image loader implementation for userspace
//! This module provides functionality for loading and saving images.

use ample::imagetic::Image;

/// The simple image format supported by our loader
///
/// Format:
/// - 4 bytes: Magic number "IMG\0"
/// - 4 bytes: Width (u32, little-endian)
/// - 4 bytes: Height (u32, little-endian)
/// - 1 byte: Bytes per pixel
/// - Remaining bytes: Pixel data (width * height * bytes_per_pixel)
struct SimpleImageFormat;

/// Load an image from a file
pub fn load_image_file(filepath: &str) -> Option<Image<ample::Origin, ample::Origin>> {
    // Use the existing file loading functionality
    let result = crate::file::load(filepath);

    match result {
        Some((fd, stat, data)) => {
            // Parse the image data
            let image = unsafe { load_from_bytes(data, stat.st_size as usize) };

            // Close the file regardless of whether we successfully loaded the image
            let _ = crate::target::os::syscall::close(fd as i32);

            // Free the file mapping memory if image loading was successful
            if image.is_some() {
                unsafe {
                    // Use our syscalls wrapper
                    let _ = crate::image::syscalls::munmap(data as *mut u8, stat.st_size as usize);
                }
            }

            image
        }
        None => None,
    }
}

/// Load image from raw bytes
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers.
pub unsafe fn load_from_bytes(
    data: *const u8,
    size: usize,
) -> Option<Image<ample::Origin, ample::Origin>> {
    // Check if we have enough bytes for the header
    if size < 13 {
        return None;
    }

    // Verify magic number
    let magic = unsafe { core::slice::from_raw_parts(data, 4) };
    if magic != [b'I', b'M', b'G', 0] {
        return None;
    }

    // Read width (little-endian u32)
    let width_bytes = unsafe { core::slice::from_raw_parts(data.add(4), 4) };
    let width = u32::from_le_bytes([
        width_bytes[0],
        width_bytes[1],
        width_bytes[2],
        width_bytes[3],
    ]) as usize;

    // Read height (little-endian u32)
    let height_bytes = unsafe { core::slice::from_raw_parts(data.add(8), 4) };
    let height = u32::from_le_bytes([
        height_bytes[0],
        height_bytes[1],
        height_bytes[2],
        height_bytes[3],
    ]) as usize;

    // Read bytes per pixel
    let bytes_per_pixel = unsafe { *data.add(12) };

    // Calculate expected data size
    let expected_size = 13 + (width * height * bytes_per_pixel as usize);
    if size < expected_size {
        return None;
    }

    // Allocate memory for the image data
    let pixel_data_size = width * height * bytes_per_pixel as usize;

    // Allocate memory using our mmap wrapper
    let image_data = unsafe { crate::image::syscalls::mmap_anonymous(pixel_data_size) };

    if image_data.is_null() {
        return None;
    }

    // Copy the pixel data
    unsafe {
        core::ptr::copy_nonoverlapping(data.add(13), image_data, pixel_data_size);

        // Create and return the image
        Some(Image::new(width, height, image_data, bytes_per_pixel))
    }
}

/// Save an image to a file
pub fn save_image_file(image: &Image<ample::Origin, ample::Origin>, filepath: &str) -> bool {
    // Calculate total file size
    let data_size = image.size_bytes();
    let total_size = 13 + data_size; // 4 (magic) + 4 (width) + 4 (height) + 1 (bpp) + data

    // Allocate memory for the file data
    // Use our mmap wrapper
    let file_data = unsafe { crate::image::syscalls::mmap_anonymous(total_size) };

    if file_data.is_null() {
        return false;
    }

    unsafe {
        // Write magic number
        let magic = [b'I', b'M', b'G', 0];
        core::ptr::copy_nonoverlapping(magic.as_ptr(), file_data, 4);

        // Write width (little-endian)
        let width_bytes = (image.width as u32).to_le_bytes();
        core::ptr::copy_nonoverlapping(width_bytes.as_ptr(), file_data.add(4), 4);

        // Write height (little-endian)
        let height_bytes = (image.height as u32).to_le_bytes();
        core::ptr::copy_nonoverlapping(height_bytes.as_ptr(), file_data.add(8), 4);

        // Write bytes per pixel
        *file_data.add(12) = image.bytes_per_pixel;

        // Write image data
        core::ptr::copy_nonoverlapping(image.data, file_data.add(13), data_size);
    }

    // Open file for writing
    let filepath_ptr = ample::string::terminate::<ample::Origin, ample::Origin>(filepath);
    // Use our open_file wrapper
    let flags = (crate::target::os::syscall::open::flags::Flag::WRONLY as i32)
        | (crate::target::os::syscall::open::flags::Flag::CREAT as i32)
        | (crate::target::os::syscall::open::flags::Flag::TRUNC as i32);

    let result = match unsafe { crate::image::syscalls::open_file(filepath_ptr, flags) } {
        Some(fd) => {
            // Use our write_file wrapper
            let write_success =
                unsafe { crate::image::syscalls::write_file(fd, file_data, total_size) };

            // Close file using our wrapper
            let _ = crate::image::syscalls::close_file(fd);

            // Return write success status
            write_success
        }
        _ => false,
    };

    // Clean up
    unsafe {
        // Free the memory using munmap
        let _ = crate::target::os::syscall::munmap(
            file_data as *mut u8 as *mut core::ffi::c_void,
            total_size,
        );
    }

    result
}
