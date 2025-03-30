use image::DynamicImage; // Importing the DynamicImage type
use image_manipulator::{resize_image, grayscale_image, crop_image};

fn main() {
    const FAILED_TO_OPEN_IMAGE: &str = "Failed to open image";
    const FAILED_TO_SAVE_IMAGE: &str = "Failed to save image";

    let img_path: &str = "input.jpg"; // Path to the input image as a string reference to avoid heap allocation

    let img: DynamicImage  = image::open(img_path).expect(FAILED_TO_OPEN_IMAGE);

    let resized_img: DynamicImage = resize_image(&img, 800, 600);
    resized_img.save("output_resized.jpg").expect(FAILED_TO_SAVE_IMAGE);

    let grayscale_img: DynamicImage = grayscale_image(&img);
    grayscale_img.save("output_grayscale.jpg").expect(FAILED_TO_SAVE_IMAGE);

    let cropped_img: DynamicImage = crop_image(&img, 100, 100, 400, 400);
    cropped_img.save("output_cropped.jpg").expect(FAILED_TO_SAVE_IMAGE);

    println!("Image processing completed!");
}
