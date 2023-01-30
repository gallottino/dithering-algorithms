use super::threshold::Threshold;
use image::{DynamicImage, ImageBuffer, Luma, Pixel};

pub fn floyd_steinberg(img: &mut DynamicImage) -> DynamicImage {
    let mut dither_image = image::GrayImage::new(img.width(), img.height());
    let mut grey_image = img.clone().grayscale().to_luma8();

    println!("{:?}", grey_image.get_pixel(121, 121).to_luma());
    for y in 0..img.height() {
        for x in 0..img.width() {
            let old_pixel = grey_image.get_pixel(x, y).to_luma();
            let new_pixel = old_pixel.threshold();

            let error = (old_pixel.0[0] as f32) - (new_pixel.0[0] as f32);

            let error_diffusion: Vec<(i32, i32, f32)> = vec![
                ((x as i32) + 1, (y as i32), (2.0 / 16.0)),
                ((x as i32) - 1, (y as i32) + 1, (7.0 / 16.0)),
                ((x as i32), (y as i32) + 1, (6.0 / 16.0)),
                ((x as i32) + 1, (y as i32) + 1, (1.0 / 16.0)),
            ];

            diffuse_error(&mut grey_image, error, error_diffusion);

            dither_image.put_pixel(x, y, new_pixel);
        }
    }
    image::DynamicImage::ImageLuma8(dither_image)
}

fn diffuse_error(
    img: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
    error: f32,
    matrix_diffusion: Vec<(i32, i32, f32)>,
) {
    for (x_i32, y_i32, perc) in matrix_diffusion {
        let x = u32::try_from(x_i32);
        let y = u32::try_from(y_i32);
        match (x, y) {
            (Ok(x), Ok(y)) => {
                if x < img.width() && y < img.height() {
                    let pixel_value = img.get_pixel(x, y).to_luma().0[0] as f32 + error * perc;
                    //println!("{:?}", pixel_value as u8);
                    img.put_pixel(x, y, Luma([pixel_value as u8]));
                }
            }
            _ => continue,
        }
    }
}
