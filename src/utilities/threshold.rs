use image::{DynamicImage, GenericImageView, Luma, Pixel};

const BLACK: Luma<u8> = Luma([0]);
const WHITE: Luma<u8> = Luma([255]);

pub trait Threshold {
    fn threshold(&self) -> Luma<u8>;
    fn threshold_by_value(&self, value: u8) -> Luma<u8>;
}

impl Threshold for Luma<u8> {
    fn threshold(&self) -> Luma<u8> {
        self.threshold_by_value(127)
    }

    fn threshold_by_value(&self, value: u8) -> Luma<u8> {
        match self.0[0] > value {
            true => WHITE,
            false => BLACK,
        }
    }
}

pub fn threshold(img: DynamicImage) -> DynamicImage {
    let mut dither_image = image::GrayImage::new(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            dither_image.put_pixel(x, y, img.get_pixel(x, y).to_luma().threshold());
        }
    }

    image::DynamicImage::ImageLuma8(dither_image)
}
