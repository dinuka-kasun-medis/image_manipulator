use image::DynamicImage;

// Converts an image to grayscale.
pub fn grayscale_image(img: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(img.to_luma8())
}
