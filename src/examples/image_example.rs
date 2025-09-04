//! Example program demonstrating image loading and saving

// The #![no_main] and #![no_std] are already applied at the crate level
// so we don't need them here in the example module

use crate::image;

// Simple example function that demonstrates image handling
pub fn run_image_example() {
    // Display a welcome message
    crate::info!("Image Handling Example\n");
    crate::info!("=====================\n");

    // First, let's create a small test image (8x8 checkerboard pattern)
    crate::info!("Creating an 8x8 test image...\n");
    let mut test_image = unsafe {
        // Create 8x8 image with 1 byte per pixel
        match image::create(8, 8, 1) {
            Some(img) => img,
            None => {
                crate::info!("Failed to create test image!\n");
                return;
            }
        }
    };

    // Fill with a simple checkerboard pattern
    unsafe {
        let data_ptr = test_image.data as *mut u8;
        for y in 0..8 {
            for x in 0..8 {
                let index = y * 8 + x;
                let value = if (x + y) % 2 == 0 { 255 } else { 0 };
                *data_ptr.add(index) = value;
            }
        }
    }

    // Save the test image
    crate::info!("Saving test image to 'checkerboard.img'...\n");
    if image::save(&test_image, "checkerboard.img") {
        crate::info!("Image saved successfully!\n");
    } else {
        crate::info!("Failed to save image!\n");
    }

    // Free the test image
    crate::info!("Freeing test image resources...\n");
    image::free(&mut test_image);

    // Try to load the image we just saved
    crate::info!("Loading 'checkerboard.img'...\n");
    let mut loaded_image = match image::load("checkerboard.img") {
        Some(img) => {
            crate::info!(
                "Image loaded successfully! Size: {}x{}, {} bytes per pixel\n",
                img.width,
                img.height,
                img.bytes_per_pixel
            );
            img
        }
        None => {
            crate::info!("Failed to load image!\n");
            return;
        }
    };

    // Verify the image content
    crate::info!("Verifying image content...\n");
    let mut errors = 0;
    unsafe {
        let data_ptr = loaded_image.data;
        for y in 0..8 {
            for x in 0..8 {
                let index = y * 8 + x;
                let expected = if (x + y) % 2 == 0 { 255 } else { 0 };
                let actual = *data_ptr.add(index);
                if actual != expected {
                    errors += 1;
                }
            }
        }
    }

    if errors == 0 {
        crate::info!("Image verification passed!\n");
    } else {
        crate::info!("Image verification failed with {} errors!\n", errors);
    }

    // Free the loaded image
    crate::info!("Freeing loaded image resources...\n");
    image::free(&mut loaded_image);

    crate::info!("Example completed successfully!\n");
}

// We don't need the _start function and panic handler here
// since this is just an example module within the userspace crate
