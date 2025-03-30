use image::{DynamicImage, imageops::FilterType};

// Resizes an image to the specified width and height.
pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, FilterType::Lanczos3)
}
