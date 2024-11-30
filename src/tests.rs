use crate::{adb_commands, cv, structs::Point};

#[test]
fn direct_adb_screenshot() {
    let image = adb_commands::screenshot().unwrap();
    cv::save_image_file("direct_adb_screenshot.png", &image).unwrap();

    println!("Execute complete, please check the image file.");
}

#[test]
fn direct_adb_tap() {
    // Settings button in control center of Mi Pad 6 Pro
    let point = Point::new(2700, 285);
    adb_commands::tap(&point).unwrap();

    println!("Execute complete, please observe device behavior.");
}

#[test]
fn cv_match_template() {
    let source = cv::load_image_file("test_assets/image_source.png").unwrap();
    let template = cv::load_image_file("test_assets/image_template.png").unwrap();

    let res = cv::cv_match_template_center(&source, &template).unwrap();

    println!("{:#?}", res);
}
