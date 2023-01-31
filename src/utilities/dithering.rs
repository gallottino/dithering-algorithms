use super::threshold::Threshold;
use image::{DynamicImage, ImageBuffer, Luma, Pixel};

pub fn floyd_steinberg(img: &mut DynamicImage) -> DynamicImage {
    let mut dither_image = image::GrayImage::new(img.width(), img.height());
    let mut grey_image = img.clone().grayscale().to_luma8();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let old_pixel = grey_image.get_pixel(x, y).to_luma();
            let new_pixel = old_pixel.threshold();

            let error = (old_pixel.0[0] as f32) - (new_pixel.0[0] as f32);

            let error_diffusion: Vec<(i32, i32, f32)> = vec![
                ((x as i32) + 1, (y as i32), (7.0 / 16.0)),
                ((x as i32) - 1, (y as i32) + 1, (3.0 / 16.0)),
                ((x as i32), (y as i32) + 1, (5.0 / 16.0)),
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

pub enum BayerSize {
    Bayer4,
    Bayer16,
}

pub fn bayer_matrix(img: &mut DynamicImage, matrix_size: BayerSize) -> DynamicImage {
    let mut dither_image = image::GrayImage::new(img.width(), img.height());
    let grey_image = img.clone().grayscale().to_luma8();

    for y in 0..img.height() {
        for x in 0..img.width() {
            if x % 2 != 0 || y % 2 != 0 {
                continue;
            }

            let bayer_matrix = generate_bayer_matrix(&matrix_size);
            for y_off in 0..2 {
                for x_off in 0..2 {
                    if x + x_off >= img.width() || y + y_off >= img.height() {
                        continue;
                    }

                    dither_image.put_pixel(
                        x + x_off,
                        y + y_off,
                        grey_image
                            .get_pixel(x + x_off, y + y_off)
                            .threshold_by_value(
                                (255.0 * bayer_matrix[y_off as usize][x_off as usize]) as u8,
                            ),
                    );
                }
            }
        }
    }
    image::DynamicImage::ImageLuma8(dither_image)
}

fn generate_bayer_matrix(matrix_size: &BayerSize) -> Vec<Vec<f64>> {
    let mut bayer_matrix;

    let bayer_4 = [[0, 2], [3, 1]];

    let bayer_16 = [
        [0, 8, 2, 10],
        [12, 14, 14, 6],
        [3, 11, 1, 9],
        [15, 17, 13, 5],
    ];

    match matrix_size {

        BayerSize::Bayer4 => {
            bayer_matrix = vec![vec![0.0; 4]; 4];
            for i in 0..2 {
                for j in 0..2 {
                    bayer_matrix[i][j] = bayer_4[i][j] as f64 / 4.0;
                }
            }
        }

        BayerSize::Bayer16 => {
            bayer_matrix = vec![vec![0.0; 16]; 16];

            for i in 0..4 {
                for j in 0..4 {
                    bayer_matrix[i][j] = bayer_16[i][j] as f64 / 16.0;
                }
            }
        }
    }

    bayer_matrix
}
