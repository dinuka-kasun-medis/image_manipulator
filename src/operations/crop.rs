use image::DynamicImage;

// Crops an image to the specified rectangular region.
pub fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    let mut img: DynamicImage = img.clone();
    DynamicImage::ImageRgba8(image::imageops::crop(&mut img, x, y, width, height).to_image())
}
