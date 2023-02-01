use image::{DynamicImage, Rgb, GenericImageView, GrayImage};

pub mod dithering;
pub mod threshold;

const BLACK: image::Luma<u8> = image::Luma([0]);
const WHITE: image::Luma<u8> = image::Luma([255]);

pub fn set_binary_color(img: &DynamicImage, black: Rgb<u8>, white: Rgb<u8>) -> DynamicImage {
    let mut new_img = image::RgbImage::new(img.width(), img.height());

    for x in 0..img.width() {
        for y in 0..img.height() {
            match img.get_pixel(x, y).0 {
                luma => {
                    if luma[0] == WHITE[0] {
                        new_img.put_pixel(x, y, white);
                    }
                    else {
                        new_img.put_pixel(x, y, black);
                    }
                }
            }
        }
    }

    image::DynamicImage::ImageRgb8(new_img)
}
