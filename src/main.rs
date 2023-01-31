mod utilities;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file_path = &args[1];
    let mut img = image::open(file_path).unwrap();

    let file_name: Vec<&str> = file_path.split(".").collect();
    let file_name = file_name[0].to_string();

    let threshold_img = utilities::threshold::threshold(img.clone());
    let mut threshold_out_path = file_name.clone();
    threshold_out_path.push_str("_threshold.png");
    threshold_img.save(threshold_out_path).unwrap();

    let greyscale_img = img.clone().grayscale();
    let mut grey_out_path = file_name.clone();
    grey_out_path.push_str("_grey.png");
    greyscale_img.save(grey_out_path).unwrap();

    let dither_image = utilities::dithering::bayer_matrix(&mut img, utilities::dithering::BayerSize::Bayer16);
    let mut dither_out_path = file_name.clone();
    dither_out_path.push_str("_dither.png");
    dither_image.save(dither_out_path).unwrap();
}
