extern crate image;
use image::ImageBuffer;
use image::Rgb;
use std::fs::File;
use image::ImageRgb8;
use image::PNG;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut image = ImageBuffer::new(nx, ny);

for (x, y, pixel) in image.enumerate_pixels_mut() {    
        let r = (std::u8::MAX as u32 * x / nx) as u8;
        let g = (std::u8::MAX as u32 * y / ny) as u8;
        let b = std::u8::MAX as u8 >> 1;
        *pixel = Rgb([r, g, b]);
    }
    
    let ref mut f = File::create("image.png").unwrap();
    ImageRgb8(image).save(f, PNG).unwrap();
}
