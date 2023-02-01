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
    Bayer64
}

impl BayerSize {
    fn from(size: u32) -> BayerSize {
        match size{
            2 => BayerSize::Bayer4,
            4 => BayerSize::Bayer16,
            8 => BayerSize::Bayer64,
            _ => todo!("BayerSize Not implemented")
        }
    }
}

pub fn bayer_matrix(img: &mut DynamicImage, matrix_size: u32) -> DynamicImage {
    let mut dither_image = image::GrayImage::new(img.width(), img.height());
    let grey_image = img.clone().grayscale().to_luma8();

    for y in 0..img.height() {
        for x in 0..img.width() {

            if x % matrix_size != 0 || y % matrix_size != 0 {
                continue;
            }

            let bayer_matrix = generate_bayer_matrix(&BayerSize::from(matrix_size));
            for y_off in 0..bayer_matrix.len() {
                for x_off in 0..bayer_matrix[0].len() {

                    let y_off = y_off as u32;
                    let x_off = x_off as u32;
                    if x + x_off >= img.width() || y + y_off >= img.height() {
                        continue;
                    }

                    let new_pixel = grey_image
                            .get_pixel(x + x_off, y + y_off)
                            .threshold_by_value(
                                (255.0 * bayer_matrix[y_off as usize][x_off as usize]) as u8,
                            );
                        
                    dither_image.put_pixel(
                        x + x_off,
                        y + y_off,
                        new_pixel
                    );
                }
            }
        }
    }
    image::DynamicImage::ImageLuma8(dither_image)
}

fn generate_bayer_matrix(matrix_size: &BayerSize) -> Vec<Vec<f64>> {
    let mut bayer_matrix;

    let bayer_4 = [[0.0, 2.0], [3.0, 1.0]];

    let bayer_16 = [
        [0.0, 8.0, 2.0, 10.0],
        [12.0, 4.0, 14.0, 6.0],
        [3.0, 11.0, 1.0, 9.0],
        [15.0, 7.0, 13.0, 5.0],
    ];

    let bayer_64 = [
        [0.0,32.0,8.0,40.0,2.0,34.0,10.0,42.0],
        [48.0,16.0,56.0,24.0,50.0,18.0,58.0,26.0],
        [12.0,44.0,4.0,36.0,14.0,46.0,6.0,38.0],
        [60.0,28.0,52.0,20.0,62.0,30.0,54.0,22.0],
        [3.0,35.0,11.0,43.0,1.0,33.0,9.0,41.0],
        [51.0,19.0,59.0,27.0,49.0,17.0,57.0,25.0],
        [15.0,47.0,7.0,39.0,13.0,45.0,5.0,37.0],
        [63.0,31.0,55.0,23.0,61.0,29.0,53.0,21.0]
    ];

    match matrix_size {

        BayerSize::Bayer4 => {
            bayer_matrix = vec![vec![0.0; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    bayer_matrix[i][j] = bayer_4[i][j] / 4.0;
                }
            }
        }

        BayerSize::Bayer16 => {
            bayer_matrix = vec![vec![0.0; 4]; 4];

            for i in 0..4 {
                for j in 0..4 {
                    bayer_matrix[i][j] = bayer_16[i][j] / 16.0;
                }
            }
        }

        BayerSize::Bayer64 => {
            bayer_matrix = vec![vec![0.0; 8]; 8];

            for i in 0..8 {
                for j in 0..8 {
                    bayer_matrix[i][j] = bayer_64[i][j] / 64.0;
                }
            }
        }
    }

    bayer_matrix
}
