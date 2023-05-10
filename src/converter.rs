use anyhow::{anyhow, Result};
use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage::ImageLuma8, GrayImage};
use std::ops::Index;

/// ASCII characters ordered by brightness
const ASCII_CHARS: &str =
    " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
/// Number of ASCII characters available
const CHARS_LEN: usize = ASCII_CHARS.len();

/// ### Convert an image to ASCII art
/// Opens the image located at `path` and converts it to ASCII art.
/// The result will have the specified `width` and `height` dimensions (in characters).
pub fn image_to_ascii(path: &str, width: u32, height: u32, invert_color: bool) -> Result<String> {
    // Open image
    let img = ImageReader::open(path)?.decode()?;
    // Resize
    let mut resized_img = img.resize_exact(width, height, FilterType::Nearest);

    // Invert color if specified
    if invert_color {
        resized_img.invert();
    }

    // Convert to grayscale
    match resized_img.grayscale() {
        ImageLuma8(gray_img) => Ok(convert_to_ascii(gray_img)?),
        _ => Err(anyhow!("Failed to convert image to grayscale")),
    }
}

/// ### Convert a grayscale image to ASCII art
/// Replaces each pixel of the image with an ASCII character corresponding to its brightness.
pub fn convert_to_ascii(img: GrayImage) -> Result<String> {
    let mut ascii = String::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img
                .get_pixel_checked(x, y)
                .ok_or(anyhow!("Pixel out of bounds"))?;

            ascii.push(get_ascii_value(*pixel.index(0)));
        }
        ascii.push('\n');
    }
    Ok(ascii)
}

/// Get the ASCII character corresponding to a brightness value
fn get_ascii_value(brightness: u8) -> char {
    let index = (brightness as f32 / 255.0 * (CHARS_LEN - 1) as f32) as usize;
    // Brightness will always be between 0 and 255 (u8)
    ASCII_CHARS.chars().nth(index).expect("Invalid brightness")
}
