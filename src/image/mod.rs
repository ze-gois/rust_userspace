//! Image handling module for userspace
//! This module provides functionality for loading and working with images.

pub mod loader;
pub mod syscalls;

use ample::imagetic::Image;

/// Loads an image from a file path
///
/// # Examples
///
/// ```
/// let image = userspace::image::load("path/to/image.img");
/// if let Some(img) = image {
///     // Use the image...
///     userspace::image::free(&mut img);
/// }
/// ```
pub fn load(filepath: &str) -> Option<Image<ample::Origin, ample::Origin>> {
    // Use the ImageHandle type to load the image
    loader::load_image_file(filepath)
}

/// Frees resources associated with an image
pub fn free(image: &mut Image<ample::Origin, ample::Origin>) {
    if !image.data.is_null() {
        unsafe {
            // Free the memory using our allocator implementation
            let layout = ample::traits::allocatable::Layout {
                size: image.size_bytes(),
                align: core::mem::align_of::<u8>(),
            };

            // Call deallocate on the data pointer
            unsafe {
                // Use our wrapper for munmap
                syscalls::munmap(image.data as *mut u8, image.size_bytes());
            }
        }

        image.data = core::ptr::null();
        image.width = 0;
        image.height = 0;
        image.bytes_per_pixel = 0;
    }
}

/// Creates a new empty image with the given dimensions
///
/// # Safety
///
/// This function is unsafe because it allocates memory and returns a raw pointer.
/// The caller is responsible for freeing the memory using the `free` function.
pub unsafe fn create(
    width: usize,
    height: usize,
    bytes_per_pixel: u8,
) -> Option<Image<ample::Origin, ample::Origin>> {
    let size = width * height * bytes_per_pixel as usize;

    // Allocate memory using our mmap wrapper
    let data = unsafe { syscalls::mmap_anonymous(size) };
    if data.is_null() {
        return None;
    }

    unsafe {
        // Zero out the memory
        core::ptr::write_bytes(data, 0, size);

        Some(Image::new(width, height, data, bytes_per_pixel))
    }
}

/// Saves an image to a file (simple format)
///
/// Returns true if successful, false otherwise.
pub fn save(image: &Image<ample::Origin, ample::Origin>, filepath: &str) -> bool {
    loader::save_image_file(image, filepath)
}

// Create a wrapper type for ImageHandle functionality
pub struct ImageHandler;

impl ImageHandler {
    /// Free an image's data
    pub fn free_image_data(image: &mut Image<ample::Origin, ample::Origin>) {
        // Call our free function
        free(image);
    }
}
