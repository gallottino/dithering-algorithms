mod utilities;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file_path = &args[1];
    let img = image::open(file_path).unwrap();

    let file_name: Vec<&str> = file_path.split(".").collect();
    let file_name = file_name[0].to_string();

    let threshold_img = utilities::threshold::threshold(img.clone());
    let mut threshold_out_path = file_name.clone();
    threshold_out_path.push_str("_threshold.png");
    threshold_img.save(threshold_out_path).unwrap();
}
