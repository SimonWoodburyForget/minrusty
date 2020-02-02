use image::io::Reader;
use image::DynamicImage;
use image::FilterType;
use std::io::Cursor;

/// Loads an image from bytes, resizes it to 32x32 to avoid dealing with varying size images.
pub fn load_bytes(bytes: &[u8]) -> DynamicImage {
    Reader::new(Cursor::new(bytes.as_ref()))
        .with_guessed_format()
        .expect("Cursor io never fails!")
        .decode()
        .unwrap()
        // TODO:
        // - handle images of varying sizes
        .resize(32, 32, FilterType::Nearest)
}
